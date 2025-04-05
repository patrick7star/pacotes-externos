#![allow(warnings)]
/* Verifica se as fontes descompactadas, correspodem a todos pacotes que já
 * foram baixados. */
use std::path::{PathBuf, Path};
use std::io::{self};
use std::ffi::{OsStr};

type Packages = Vec<PathBuf>;


fn apenas_filtra_arquivos_da_extensao(raiz:&Path, extensao: &OsStr, 
  lista: &mut Packages) -> io::Result<usize>
{
   let bate = Some(extensao);

   for entry in raiz.read_dir()? {
      let caminho = entry?.path();

      if caminho.is_file() && caminho.extension() == bate {
         lista.push(caminho);      
      } else if caminho.is_dir() {
         let aplicacao_da_funcao = apenas_filtra_arquivos_da_extensao;
         let valor = aplicacao_da_funcao(&caminho, extensao, lista)?;

         if valor % 15 == 0
            { println!("Já iteramos no total ... {}", valor); }
      } else if caminho.is_symlink() 
         { println!("Ignorando linque simbólico {caminho:?}"); }
   }

   Ok(lista.len())
}

fn filtras_arquivos_do_tipo(raiz: &Path, extensao: &str) 
  -> io::Result<Packages>
{
   let mut output = Packages::new();
   let aplicacao_da_filtragem = apenas_filtra_arquivos_da_extensao;
   let extensao = OsStr::new(extensao);
   let _= aplicacao_da_filtragem(raiz, extensao, &mut output)?;

   Ok(output)
}

fn total_de_bytes(lista: &Packages) -> u64
{
   lista.iter().map(|pkg| pkg.metadata()).filter(|In| In.is_ok())
   .map(|In| In.unwrap().len()).sum()
}

#[cfg(test)]
mod tests {
   extern crate utilitarios;
   extern crate toml;

   use super::*;
   use std::io::{Read};
   use std::fs::{File};
   use utilitarios::aleatorio::{sortear};
   use utilitarios::legivel::{tamanho};
   use toml::{from_str, value::{Value, Table}};

   #[test]
   fn apenasFiltraArquivosDaExtensao() {
      let root = Path::new(env!("HOMEPATH")).join(".cargo/registry");
      let out = filtras_arquivos_do_tipo(&root, "crate");
      let size = total_de_bytes(out.as_ref().unwrap());

      println!("O total foi de {}.", out.as_ref().unwrap().len());
      for pth in out.as_ref().unwrap() 
         { println!("\t- {}", pth.display()); }
      
      println!("Eles totalizam em {} bytes", tamanho(size, true));
   }

   fn escolhe_toml_aleatorio_do_sistema<'a>(total: &'a Packages) 
     -> Option<&'a Path> { 
      let ultimo = total.len() - 2;
      Some(&total[ultimo / 2]) 
   }
   
   #[test]
   fn verificandoDepedenciasViaTomlFile() {
      let base = env!("HOMEPATH");
      let root = Path::new(base).join(".cargo/registry/src");
      let list = filtras_arquivos_do_tipo(&root, "toml").unwrap();

      for item in list.iter() { 
         let projeto = item.parent().unwrap().file_name().unwrap();
         println!("{:.<13?}", projeto); 
      }
      let size = tamanho(total_de_bytes(&list), true);
      println!("Totalizando em {} bytes", size);
      
      let choice = escolhe_toml_aleatorio_do_sistema(&list).unwrap();
      println!("Seleção da amostra: {:?}", choice);

      let mut buffer = String::new();
      let mut stream = File::open(choice).unwrap();
      let obj = from_str::<Table>(&buffer);

      println!("{obj:#?}");
   }
}
