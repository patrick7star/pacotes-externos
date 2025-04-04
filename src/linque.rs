/* cria link simbólico tanto para a versão em debug, quanto para o 
 * binário final. */
#[cfg(target_os="linux")]
use std::os::unix::fs::symlink;
#[cfg(target_os="windows")]
use std::os::windows::fs::{symlink_file as symlink};
use std::env::current_exe;
use std::path::PathBuf;
use std::ffi::{OsStr};
use std::path::{Path, Component};
use std::env::{var, VarError};
use std::io;

// Nome do programa aqui.
const NOME_DO_PROGRAMA: &'static str = "cargo-listagem.exe";


/// Computa o caminho até o projeto, baseado que, tal executável deve está
/// no lugar comum onde uma simples compilação do Rust o faz se não for 
/// definida diferente. Portanto em algum subdiretório do diretório 'target'.
pub fn computa_caminho(caminho_str:&str) -> PathBuf {
   match current_exe() {
   // à partir do caminho do executável ...
      Ok(mut base) => {
         // remove executável do caminho.
         base.pop(); 
         // sai do subdiretório 'release'.
         base.pop(); 
         // sai do subdiretório 'target'.
         base.pop();
         // complementa com o caminho passado.
         base.push(caminho_str);
         return base;
      } Err(_) =>
         { panic!("não foi possível obter o caminho do executável!"); }
   }
}

fn cria_linques_no_repositorio(nome_do_linque: &str) -> io::Result<PathBuf> 
{
   let caminho_do_executavel = current_exe()?;
   let caminho_repositorio =  match var("LINKS") {
      Ok(data) => Ok(data),
      Err(tipo_de_erro) => 
      {
         let erro_a = io::ErrorKind::InvalidInput;
         let erro_b = io::ErrorKind::InvalidData;

         match tipo_de_erro {
            VarError::NotPresent => Err(erro_a),
            VarError::NotUnicode(_) => Err(erro_b)
         }
      }
   }?;
   let mut fonte = caminho_do_executavel;
   let destino = Path::new(&caminho_repositorio).join(nome_do_linque);
   let bate = Component::Normal(OsStr::new("release"));

   if destino.exists()
      { return Err(io::ErrorKind::AlreadyExists.into()); }

   // Verificação se estamos nos referindo apenas da parte 'release'.
   if fonte.components().any(|part| part == bate) 
   {
      // Corrigindo para atual caminho se for um linque simbólico.
      if fonte.is_symlink() 
         { fonte = fonte.read_link()?; }

      symlink(&fonte, &destino)?;
      Ok(destino)
   } else
      { Err(io::ErrorKind::Unsupported.into()) }
}

fn cria_linques_locais(nome_do_linque: &str) -> io::Result<PathBuf>
{
   let (fonte, destino): (PathBuf,PathBuf);

   // Seleção baseado no tipo de optimização na compilação:
   if cfg!(debug_assertions) 
   {
      let novo_nome = format!("{}-debug", nome_do_linque);
      let antigo_nome = format!("target/debug/{}", NOME_DO_PROGRAMA);

      // fonte = computa_caminho("target/debug/limpa_downloads");
      fonte = computa_caminho(&antigo_nome);
      destino = computa_caminho(&novo_nome);

   } else {
      let antigo_nome = format!("target/release/{}", NOME_DO_PROGRAMA);

      fonte = computa_caminho(&antigo_nome);
      destino = computa_caminho(nome_do_linque);
   }

   // Escolhe a criação do linque, baseado no tipo de execução aplicada.
   symlink(fonte, &destino)?;
   // Retorno do linque que acabou de ser criado.
   Ok(destino)
}

pub fn linca_executaveis(nome_do_linque: &str) 
{
   match cria_linques_locais(nome_do_linque) {
      Ok(_) => 
         { println!("O linque local foi criado com sucesso."); }
      Err(erro) => match erro.kind() {
         io::ErrorKind::AlreadyExists => {
            if cfg!(debug_assertions)
               { println!("Já existe um linque local do 'modo debug'."); }
            else 
               { println!("Já existe um linque local."); }
         } _ =>
         // Demais erros ainda não tratados.
            { panic!("{}", erro); }
      }
   };

   match cria_linques_no_repositorio(nome_do_linque) {
      Ok(caminho) => { 
         assert!(caminho.exists()); 
         println!("Linque criado com sucesso em $LINKS."); 
      } Err(classificacao_do_erro) => { 
         match classificacao_do_erro.kind() {
            io::ErrorKind::AlreadyExists =>
               { println!("Já existe um linque em $LINKS."); }
            io::ErrorKind::Unsupported =>
               { println!("Sistema ou ambiente não suportado.");}
            _ =>
               { panic!("{}", classificacao_do_erro); }
         }
      } 
   }
}
