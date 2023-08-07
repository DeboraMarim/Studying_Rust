/*
Testes de integração (integration tests) em Rust são usados para testar a interação e a colaboração entre várias partes do código em um cenário mais amplo. Enquanto os testes unitários se concentram em verificar a correção de unidades individuais, os testes de integração verificam como essas unidades trabalham juntas como um todo.

Em Rust, os testes de integração são organizados em arquivos separados na pasta tests/ do projeto. Cada arquivo de teste representa um cenário de teste independente e pode chamar funções e módulos do código principal para testá-los em conjunto.

Aqui está um exemplo de como escrever um teste de integração em Rust:

Suponha que temos uma função sum que soma dois números e uma função multiply que multiplica dois números. Vamos escrever um teste de integração para verificar se a soma e a multiplicação estão funcionando corretamente juntas:

Crie um novo arquivo no diretório tests/ com a extensão .rs. Por exemplo, integration_test.rs.

Escreva o código de teste nesse arquivo, incluindo os testes que deseja realizar.

*/

// Arquivo de teste de integração (tests/integration_test.rs)

use my_project::sum;
use my_project::multiply;

#[test]
fn test_sum_and_multiply() {
    // Teste de soma
    let result_sum = sum(2, 3);
    assert_eq!(result_sum, 5);

    // Teste de multiplicação
    let result_multiply = multiply(4, 5);
    assert_eq!(result_multiply, 20);
}


/*

Neste exemplo, supomos que temos um projeto chamado my_project, onde as funções sum e multiply estão definidas. O arquivo integration_test.rs importa essas funções usando use, permitindo que os testes sejam executados.

Para rodar os testes de integração, você pode executar o seguinte comando:  

cargo test --test integration_test


O Cargo identificará automaticamente o arquivo de teste de integração e o executará como parte dos testes do projeto.

Os testes de integração são uma parte importante do processo de garantia de qualidade em Rust. Eles ajudam a verificar se as diferentes partes do código funcionam corretamente em conjunto, garantindo que o software como um todo está se comportando como esperado.

*/

