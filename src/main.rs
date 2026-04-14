use std::collections::HashMap;


#[derive(Debug, Clone)]
struct Produto {
    id: u32,
    nome: String,
    marca: String,
    categoria: String,
}

struct SistemaBusca {
     
    indice: HashMap<String, Vec<Produto>>,
}

impl SistemaBusca {
    fn novo() -> Self {
        SistemaBusca {
            indice: HashMap::new(),
        }
    }

    
    fn adicionar_produto(&mut self, produto: Produto) {
        let nome_chave = produto.nome.to_lowercase();
        self.indice.entry(nome_chave).or_insert(Vec::new()).push(produto);
    }

    
    fn buscar(&self, termo: &str) -> Option<&Vec<Produto>> {
        self.indice.get(&termo.to_lowercase())
    }
}

fn main() {
    let mut sistema = SistemaBusca::novo();

    sistema.adicionar_produto(Produto {
        id: 1,
        nome: "Teclado Mecanico".to_string(),
        marca: "Logitech".to_string(),
        categoria: "Eletronicos".to_string(),
    });

    sistema.adicionar_produto(Produto {
        id: 2,
        nome: "Cadeira Gamer".to_string(),
        marca: "Racer".to_string(),
        categoria: "Moveis".to_string(),
    });

    
    let termo = "Teclado Mecanico";
    println!("Buscando por: {}", termo);

    match sistema.buscar(termo) {
        Some(produtos) => {
            for p in produtos {
                println!("Produto encontrado: {:?} em tempo recorde!", p);
            }
        }
        None => println!("Produto não encontrado."),
    }
}