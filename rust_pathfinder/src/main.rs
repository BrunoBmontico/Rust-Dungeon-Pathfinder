// Faça as inicializaçoes apontadas([p,d] na figura)
// 
// Enquanto houver vertice aberto:
// Escolha u cuja estimativa seja a menor dentre os abertos
// Feche u
// Para todo no aberto v na adjacencia de u:
// relaxe a arestra (u,v)

use std::collections::BinaryHeap;
use std::cmp::Reverse;

// funçao vai receber uma matriz bi dimencional que recebe um usize
// esse usize tem que ser um u32 ou seja um inteiro nao negativo de 32bits
// tambem vai receber um valor de origem, que vai definir o ponto inicial do grafo
// ou seja seu vertice de menor valor possivel, 0.
// funçao vai retornar um array de numeros inteiros de 32bits,
// provavelmente sendo o caminho percorrido.
fn dijkstra(grafo: &Vec<Vec<(usize, u32)>>, origem: usize) -> Vec<u32> {

    // variavel declarada para caucular o tamanho da matriz recebida
    let n = grafo.len();

    // define o valor infinito dentro de 32bits por cada vertice
    // ate chegar no tamanho da matriz
    let mut dist = vec![u32::MAX; n];

    // aloca um espaço heap na memoria que vai receber a matriz.
    // heap.push(Reverse((0u32, origem))); empurra o primeiro valor da lista do heap
    // sendo igual a origem, ou seja onde do grafo vc vai começar e o seu valor como 0,
    // em um caso real seria como dizer a sua propria posiçao ao google maps,
    // vc esta a 0 km de vc mesmo
    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0u32, origem)));

    // enquanto existir um vertice menor dentre os abertos(heap.pop junto do reverse)
    while let Some(Reverse((d,u))) = heap.pop() {

        // ainda n~ao compreendi
        if d > dist[u] { continue; }

        // para cada vetor e seu peso dentro da matriz
        // uma nova distancia e seleciona igual ao u(vizinhos),
        // somando com seu peso, seguindo com as contas basicas do algoritimo para saber
        // o menor dos valores.
        for &(v, peso) in &grafo[u] {
            let nova_dist = dist[u] + peso;
            if nova_dist < dist[v] {
                dist[v] = nova_dist;
                heap.push(Reverse((nova_dist, v)));
            }
        }

    }
    dist
}

fn main() {
    
}
