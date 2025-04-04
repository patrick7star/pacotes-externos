/** 
   Este programa tem como objetivo substituir a listagem de libs externas do
 Rust, estas que foram baixados por outras bibliotecas na hora de 
 compilação, ou adicionadas por mim mesma de modo manual. Claro que será uma
 coisa bem mais organizada do que o simples comando faz hoje, usando de 
 sintaxe colorida e organizando versões numa simples linha.
*/

mod linque;

use std::collections::HashMap;
use std::io::{Result as ResultadoIO};
use std::path::{Path, PathBuf};
use std::fs::read_dir;
// Outros módulos deste projeto:
use crate::linque::{linca_executaveis};

type Pacote = HashMap<String, Vec<String>>;
type Par = (String, Vec<String>);
type Pares = Vec<Par>;


#[cfg(target_os="linux")]
const RAIZ: &'static str = concat!(
   env!("HOME"), 
   "/.cargo/registry/src"
);

#[cfg(target_os="windows")]
const RAIZ: &'static str = concat!(
   env!("HOMEPATH"), 
   "/.cargo/registry/src"
);

fn todos_diretorios_fontes() -> ResultadoIO<Vec<PathBuf>> {
   // bolsa para coletar caminhos de códigos-fontes.
   let mut coletador = Vec::<PathBuf>::new();

   /* iterando os iniciais 2 diretórios, mais de uma forma
    * que avance dentro deles, vasculhando os diretórios
    * fontes de terceiros. */
   for pth in read_dir(RAIZ)? {
      let caminho = pth?.path();
      /* se for algum diretório, entrá nele, e finalizar o
       * trabalho(colocar um código-fonte na lista). */
      if caminho.as_path().is_dir() {
         for pth_inner in read_dir(caminho.as_path())? {
            let fonte = pth_inner?.path();
            if fonte.as_path().is_dir()
               { coletador.push(fonte); }
         }
      } 
   }

   /* enviando de volta lista com possíveis códigos-fontes 
    * coletados nele.*/
   Ok(coletador)
}

fn identificando_fonte<'a>(codigo_fonte: &'a Path) 
  -> (&'a str, &'a str)
{
   /* separa o nome da versão do código-fonte passado.*/
   // nome do diretório em sí.
   let diretorio_nome = codigo_fonte.file_name().unwrap();
   /* divindo-o através do traço que separa o nome da versão. */
   diretorio_nome.to_str().unwrap()
   .rsplit_once('-').unwrap()
}

fn organizando_fontes_e_suas_versoes() -> Option<Pacote> {
   /* agloremando códigos-fontes iguais, porém com versões diferentes
    * deles. O retorno será um dicionário, onde a chave será o nome
    * da fonte, já os valores serão uma array contendo todas versões
    * disponíveis no sistema. Os resultado pode ser válido ou não, 
    * dependendo se há algo no sistema.
    */
   let mut tudo = todos_diretorios_fontes().unwrap();

   /* se estiver vázio, ou seja, não há nenhuma biblioteca de terceiros no 
   computador, apenas retorna sem dados(none). */
   if tudo.is_empty() { return None; }

   /* dicionário contendo nome do código, e suas versões disponíveis. */
   // let mut compilado: HashMap<String, Vec<String>>;
   let mut compilado = Pacote::with_capacity(tudo.len());

   // inserindo organizadamente(não quer dizer ordenado) no dicionário ...
   for caminho in tudo.drain(..) {
      let (nome, versao) = identificando_fonte(&caminho);
      let versao_str = versao.to_string();

      if compilado.contains_key(nome) {
         let entrada = compilado.get_mut(nome).unwrap();
         /* evitando a mesma versão de ser adicionada novamente, por algum 
          * motivo tal redundância está acontencendo. Algum erro bobo com 
          * a listagem acima, porém está sendo resolvido aqui na inserção 
          * do mapa. */
         if !entrada[..].contains(&versao_str)
            { entrada.push(versao.to_string()); }
         
      } else {
         compilado.insert(
            nome.to_string(), 
            vec![versao.to_string()]
         );
      }
   }

   /* retornando o mapa contendo chaves(códigos-fontes),
    * e suas versões. */
   Some(compilado)
}

fn listagem_das_fontes(repositorio: Pacote) {
   /* cuidando especialmente da função de visualização de todos estes
    * dados, formando a saída para o modo mais legível possível.
    */
   if repositorio.is_empty() 
      { println!("\nnão há nada aqui!\n"); return (); }

   // baseando a formatação no comprimento da maior string.
   let maior_comprimento = {
      repositorio.keys()
      .map(|s| s.len())
      .max().unwrap() + 4
   };

   /* boa listagem de das versões, que se preocupa em arrancar a última
    * vírgula. */
   fn listagem_versoes(mut l: Vec<String>) {
      let total = l.len();

      for (i, numero) in l.drain(..).enumerate() {
         if i == (total - 1)
            { print!("v{}", numero); }
         else
            { print!("v{} | ", numero); }
      }
   }

   for (nome, versoes) in ordena_repositorio(repositorio).drain(..) {
      print!("\t{0:.<maior_comprimento$} \u{27e8}", nome);
      listagem_versoes(versoes);
      println!("\u{27e9}");
   }
}

fn ordena_repositorio(mut repositorio: Pacote) -> Pares {
   /* Ordena uma lista de strings de a até z, baseado na chave do mapa. */
   let mut array = repositorio.drain().collect::<Pares>(); 

   // algoritmo de ordenação bubblesort.
   for i in 0..array.len() {
      for j in (i + 1)..array.len() {
         if array[j].0 < array[i].0 {
            // swap clonando dados, faz possível, porém bastante pesado.
            let ptr: *mut Par = array.as_mut_ptr();
            unsafe {
               ptr.add(j).swap(ptr.add(i));
            }
         }
      }
      // ordenando suas versões, já que aqui itera todos.
   }

   /* array contendo tuplas do nome mais array com versões, ordenado
    * baseado no seu nome, ou seja, primeiro elemento da tupla. */
   array
}

fn main() {
   /* preciso apenas chamar a função 'organizando_fontes_e_suas_versoes',
    * para obter o mapa com os dados, e organizar-lô, de maneira bem
    * mais bonita, do que sua versão alinhada de 'debug'.
    */
   let todo_repositoiro = organizando_fontes_e_suas_versoes();

   println!(
      "\nDepedências baixadas no computador({} \
      pacotes distintos, {} no total):",
      todo_repositoiro.as_ref().unwrap().len(),
      // soma de todas versões, é o total de pacotes baixados.
      todo_repositoiro.as_ref().unwrap()
      .iter().map(|(_, array)| array.len())
      .sum::<usize>()
   );
   listagem_das_fontes(todo_repositoiro.unwrap());
   
   // barra de termino de página.
   println!("\n{}\n", &"-".repeat(60));
   linca_executaveis("pacotes-externos");
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
   extern crate os_info;
   use super::*;
   use std::env::var;

   #[test]
   fn verificando_se_filtra_fontes_apenas() {
      let possiveis_sources = todos_diretorios_fontes(); 

      for path in possiveis_sources.unwrap() {
         let caminho = path.join("./Cargo.toml");

         // visualizando caminho.
         println!("{}", caminho.display());

         /* se todos tiverem um arquivo 'Cargo.toml', estão a meio
          * caminho de se confirmarem com um verdadeiro código-fonte
          * de Rust.*/
         assert!(caminho.exists());
      }
   }

   #[test]
   fn separacao_perfeita_das_fontes() {
      let possiveis_sources = todos_diretorios_fontes(); 

      for path in possiveis_sources.unwrap() {
         let (nome, versao) = identificando_fonte(path.as_path());
         // visualizando caminho.
         println!(
            "'{}'\nEntão após aplicado a função =>
            \r\tnome: {nome}
            \r\tversão: {versao}
            ", path.display(),
         );

      }

      // avaliação manual.
      assert!(true);
   }

   #[test]
   fn informacao_compilada_nomes_e_versoes() {
      for entrada in organizando_fontes_e_suas_versoes().unwrap() 
         { println!("{:?}", entrada);  }

      // avaliação manual.
      assert!(true);
   }

   #[test]
   fn novo_tipo_de_visualizacao() {
      let funcao: fn() -> Option<Pacote>;
      funcao = organizando_fontes_e_suas_versoes;
      let todo_repositoiro = funcao().unwrap();
      listagem_das_fontes(todo_repositoiro);
      // uma avalização manual?
      assert!(true);
   }

   #[test]
   fn comparacao_strings() {
      assert!("abacate" < "abacaxi");
   }

   #[test]
   #[ignore="apenas um teste da biblioteca"]
   fn que_tipo_de_informacao_fornece() {
      let X = os_info::get();
      const Y: &str = " --- ";

      println!(
         "Informações sobre a máquina:\n\tArquitetura: {}\n\t \
         Sistema do Tipo:{:?}\n\tCodename do OS: {}\n\tEdição: '{}'\n\t \
         Tipo de OS: {:?}\n\tVersionamento: {:?}",
         X.architecture().unwrap_or(Y), X.bitness(), 
         X.codename().unwrap_or(Y), X.edition().unwrap_or(Y),
         X.os_type(), X.version()
      );
   }

   #[test]
   #[ignore="simples verificação de ambiente"]
   fn variavel_de_ambiente_existente() {
      match var("LINKS") {
         Ok(data) =>
            { println!("LINKS: {data:}"); }
         Err(erro) => { 
            println!("Está é a mensagem de erro: '{erro:?}'"); 
            assert!(false);
         }
      }
   }
}
