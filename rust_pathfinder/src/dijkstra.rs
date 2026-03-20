use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn dijkstra(
    matriz: &Vec<Vec<(usize, u32)>>,
    origem: usize
) -> Vec<u32> {

    let tamanho_matriz = matriz.len();

    let mut predicao_distancia = vec![
        u32::MAX; 
        tamanho_matriz
    ];

    let mut array_alocado = BinaryHeap::new();
    array_alocado.push(Reverse((0u32, origem)));

    while let Some(Reverse((distancia, vertice))) = array_alocado.pop() {

        if distancia > predicao_distancia[vertice] {
            continue;
        }

        for &(indice, peso) in &matriz[vertice] {
            let confirmacao_predicao = predicao_distancia[vertice] + peso;

            if confirmacao_predicao < predicao_distancia[indice] {
                predicao_distancia[indice] = confirmacao_predicao;
                array_alocado.push(Reverse((confirmacao_predicao)));
            }
        }
    }
    predicao_distancia

}