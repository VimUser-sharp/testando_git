# Como Funcionam os Ponteiros em Rust
*Um guia para quem nunca viu ponteiros antes*

## 1️⃣ O que é um ponteiro?
- **Ponteiro** = endereço de memória onde um valor está armazenado.
- Em vez de guardar o valor direto, guardamos **“onde”** ele está.
- Isso permite compartilhar, modificar ou “referenciar” o mesmo dado em vários lugares.

## 2️⃣ Por que Rust tem tipos de ponteiro diferentes?
Rust foi criado para ser **seguro** sem precisar de um coletor de lixo (GC). Para garantir isso, ele classifica os ponteiros em duas categorias principais:

| Tipo de ponteiro | Quando usar | Quem controla a validade |
|--------------------------------------|------------------------------------------|---------------------------|
| `&T` (referência imutável) | Apenas leitura | O compilador (borrow checker) |
| `&mut T` (referência mutável) | Leitura + escrita | O compilador (borrow checker) |
| `*const T` / `*mut T` (ponteiro bruto) | Interoperabilidade C, código “unsafe” | Você (programador) |
| `Box` (alocação na heap) | Propriedade única, alocação dinâmica | O compilador (RAII) |
| `Rc` / `Arc` (contadores de referência) | Compartilhamento de propriedade (single‑thread / multi‑thread) | Contadores internos |

> **Nota:** As referências (`&`/`&mut`) são *não‑nuláveis* e sempre apontam para algo válido. Os ponteiros brutos podem ser nulos ou “pendentes”, por isso exigem `unsafe`.

## 3️⃣ Referências Imutáveis – `&T`
```rust
fn main() {
    let x = 10; // x está na stack
    let r = &x; // r é uma referência imutável a x
    println!("r = {}", r);
}
```
* Regra de empréstimo: enquanto houver uma referência imutável, não pode existir referência mutável ao mesmo valor.
* Múltiplas referências imutáveis são permitidas simultaneamente.

## 4️⃣ Referências Mutáveis – `&mut T`
```rust
fn increment(v: &mut i32) {
    *v += 1; // * desreferencia o ponteiro
}

fn main() {
    let mut num = 5;
    increment(&mut num);
    println!("num = {}", num); // 6
}
```
* Só pode existir uma referência mutável ao mesmo tempo.
* Essa restrição impede condições de corrida (race conditions) em código seguro.

## 5️⃣ Box – Alocação na Heap
```rust
fn main() {
    let b = Box::new(42); // aloca 42 na heap e devolve um ponteiro proprietário
    println!("{}", b);
}
// b sai de escopo → memória liberada automaticamente
```
* Box possui ownership exclusiva. Quando o Box cai fora do escopo, o destrutor (Drop) libera a memória.
* Útil quando o tamanho não é conhecido em tempo de compilação ou queremos mover dados para a heap.

## 6️⃣ Contadores de Referência – `Rc` e `Arc`
```rust
use std::rc::Rc;

fn main() {
    let a = Rc::new(5);
    let b = Rc::clone(&a); // aumenta o contador interno
    println!("a = {}, b = {}", a, b);
}
// ambos caem fora do escopo → contador chega a zero → memória liberada
```
* `Rc`: para uso em single‑thread. Permite múltiplas owners.
* `Arc`: versão thread‑safe (atomic). Use quando houver threads.

## 7️⃣ Ponteiros Brutos – `*const T` / `*mut T`
```rust
fn main() {
    let mut x = 10;
    let p: *mut i32 = &mut x; // ponteiro mutável bruto
    unsafe {
        *p = 20; // precisamos de bloco unsafe!
    }
    println!("x = {}", x); // 20
}
```
* Não há verificação de validade; o compilador assume que você sabe o que faz.
* Necessário para chamadas FFI (C), manipulação de memória avançada, etc.

## 8️⃣ Quando usar cada tipo?
| Situação | Melhor escolha |
|--------------------------------------|------------------------------------------|
| Ler um valor sem modificá‑lo | `&T` |
| Modificar um valor localmente | `&mut T` |
| Transferir propriedade para outra parte | `Box` |
| Compartilhar leitura entre várias partes (single‑thread) | `Rc` |
| Compartilhar leitura entre threads | `Arc` |
| Interagir com código C ou fazer operações de baixo nível | ponteiros brutos (`*const`/`*mut`) dentro de `unsafe` |

## 9️⃣ Boas práticas de segurança
1. Prefira referências (`&`/`&mut`) sempre que possível – elas são verificadas em tempo de compilação.
2. Limite o escopo dos ponteiros mutáveis: quanto menor, menos chance de bugs.
3. Evite `unsafe` a menos que seja estritamente necessário.
4. Use `Box`/`Rc`/`Arc` ao precisar de alocação dinâmica ou compartilhamento de propriedade.
5. Nunca derefencie um ponteiro nulo – isso causa undefined behavior.
