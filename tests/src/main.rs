/* O framework de testes integrado do Rust torna fácil escrever e executar testes unitários.

Para escrever testes unitários em Rust, você cria um módulo separado dentro do seu arquivo de código, anotado com o atributo #[cfg(test)]. Esse módulo é compilado e executado apenas ao rodar os testes, e permite que você escreva funções de teste usando o atributo #[test].

Aqui está um guia passo a passo para escrever testes unitários em Rust:

Primeiro, crie um novo projeto Rust ou use um já existente. Para simplificar, usaremos um único arquivo Rust para este exemplo.

Adicione o atributo #[cfg(test)] para habilitar o módulo de teste.

Escreva funções de teste com o atributo #[test] para especificar os testes unitários. Geralmente, as funções de teste têm nomes que começam com "test".

Use as macros assert! ou assert_eq! para verificar se o comportamento esperado do código coincide com os resultados reais.

Aqui está um exemplo de como escrever testes unitários para uma função simples:

*/

// Arquivo de código (exemplo.rs)

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        // Caso de teste 1
        let resultado1 = add(5, 10);
        assert_eq!(resultado1, 15);

        // Caso de teste 2
        let resultado2 = add(-8, 3);
        assert_eq!(resultado2, -5);
    }
}


/*

Neste exemplo, temos uma função add que soma dois números inteiros. Em seguida, criamos um módulo de teste com o atributo #[cfg(test)] e, dentro dele, escrevemos uma função de teste chamada test_add.

Usamos a macro assert_eq! para verificar se os resultados da chamada da função correspondem aos valores esperados. Se o resultado real e o valor esperado não forem iguais, o teste falhará e fornecerá um feedback.

Para executar os testes, você pode usar o seguinte comando:

cargo test


O Cargo detectará automaticamente e executará todos os testes unitários em seu projeto. Ele exibirá os resultados dos testes, indicando se cada teste foi aprovado ou falhou.

Escrever testes unitários ajuda a garantir que o seu código se comporte corretamente em diferentes cenários e facilita a refatoração e a manutenção da base de código com confiança. É uma prática valiosa para adotar em seu fluxo de trabalho de desenvolvimento em Rust.

*/