// Usage :
// Vec : Resizable arary. Heap-allocated array. Meilleure performance quand ajout à la fin. Bien pour une stack
// VecDeque : "vector double ended queue".Bonne performane pour une ajout à la fin et au début aussi. BIen pour une queue donc
// LinkedList : TOUJOURS une double liste chainée. Optimisé pour les suppressions et split. Bien aussi quand vraiment on a pas idée de la taille finale de la liste
// Hashmap : Bien pour faire un dictionnaire, pour un faire un cache
// BtreeMap : Pareil mais toujours trié par la clé
// Set : Bien pour une liste ensembliste ou juste pour faire de l'unicité
// BinaryHeap : Bien pour une gestion de liste de priorité

use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};

pub fn using_collections() {
    using_vec_deque();
    using_linked_list();
    using_binary_heap();
    using_hash_map();
    using_hash_set();
}

fn using_vec_deque() {
    println!("### VecDeque : ");

    let mut vecdeq = VecDeque::<i32>::new();
    vecdeq = VecDeque::<i32>::with_capacity(5);

    let vec_classic: Vec<i32> = (0..5).collect();
    vecdeq = VecDeque::from(vec_classic);

    vecdeq.push_back(5);
    vecdeq.push_front(-1);
    vecdeq.pop_front();

    vecdeq.insert(3, 10);
    vecdeq.remove(3);

    println!("{:?}", vecdeq);
}

fn using_linked_list() {
    let mut linked = LinkedList::from([1, 6, 3, 9, 7, 4]);
    linked.push_front(12);
}

fn using_binary_heap() {
    println!("### BinaryHeap : ");

    let mut binary_heap = BinaryHeap::new();
    binary_heap.push(10);
    binary_heap.push(4);
    binary_heap.push(7);
    binary_heap.push(12);
    binary_heap.push(5);
    binary_heap.push(14);

    println!("{:?}", binary_heap);

    let max = binary_heap.peek();
    if let Some(x) = max {
        println!("max : {}", x);
    }
}

fn using_hash_map() {
    let mut hashmap = HashMap::new();

    hashmap.insert("A", "Arménie".to_string());
    hashmap.insert("F", "France".to_string());

    println!("{:?}", hashmap);

    if !hashmap.contains_key("E") {
        println!("La Hashmap ne contient pas de pays avec la lettre E !")
    }

    println!("F : {}", hashmap["F"]);
    hashmap.remove("A");

    for (key, value) in &hashmap {
        println!("Key {}, Value {}", key, value);
    }

    // BTreeMap
    // Comme tout Binary Tree, les enfants de chaque noeud sont toujours par défaut inférieur à leur parent direct
    // Ca agit donc comme un excellent moyen d'avoir un type de Collection optimisé pour la recherche du max
    let mut btreemap = BTreeMap::new();
    btreemap.insert( 
        "D".to_string(), 
        "Danemark".to_string(), 
    );
}

// Permet d'avoir d'une part une liste avec des valeurs uniques
// Permet aussi de travailler sur des ensembles (union, intersection)
fn using_hash_set() {
    let mut hashset = HashSet::new();
    hashset.insert("A");
    hashset.insert("B");
    hashset.insert("C");

    println!("{}", hashset.contains("B"));

    let mut hashset_2 = HashSet::new();
    hashset_2.insert("B");
    hashset_2.insert("C");

    hashset.intersection(&hashset_2); // les deux sont équivalents
    &hashset & &hashset_2;

    hashset.union(&hashset_2);
    hashset.difference(&hashset_2);
    hashset.symmetric_difference(&hashset_2);
    hashset.is_disjoint(&hashset_2);
    hashset.is_subset(&hashset_2);
    hashset.is_superset(&hashset_2);

    // BTreeSet
    // Au même titre que les autres, l'arbre binaire permet surtout de garder un ordre de supériorité (et donc de priorité de traitement potentiellement)
    let mut btreeset = BTreeSet::new();
    btreeset.insert(0);
}
