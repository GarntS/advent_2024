/* file:    main.rs
 * author:  garnt
 * date:    12/28/2024
 * desc:    Advent of Code 2024 day 9.
 */

/// Represents a single filesystem block
#[derive(Clone, Copy, PartialEq, Debug)]
struct Block {
    pub is_empty: bool,
    pub id: usize,
    pub offset: usize,
    pub len: usize,
}

/// Represents a Doubly-linked List
#[derive(Clone, PartialEq, Debug)]
struct DblLinkedList<'a, T>
where
    T: Clone,
{
    head: Option<Box<DblLinkedListNode<'a, T>>>,
}

impl<'a, T> IntoIterator for DblLinkedList<'a, T>
where
    T: Clone,
{
    type Item = T;
    type IntoIter = DblLinkedListIntoIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        DblLinkedListIntoIterator {
            cur_node: self.head.,
        }
    }
}

// and we'll implement FromIterator
impl<'a, T> FromIterator<T> for DblLinkedList<'a, T>
where
    T: Clone,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list_head: Option<Box<DblLinkedListNode<'a, T>>> = None;
        let mut list_tail: Option<*mut Box<DblLinkedListNode<'a, T>>> = None;
        for i in iter {
            // if no head, set it. otherwise, append to tail
            unsafe {
                let mut new_node: Box<DblLinkedListNode<'a, T>> = Box::from(DblLinkedListNode {
                    prev: Some(list_tail.unwrap() as *mut DblLinkedListNode<'a, T>),
                    next: None,
                    value: i,
                });

                if let Some(tail) = list_tail {
                    (*list_tail.unwrap()).next = Some(new_node);
                    (*tail).next.as_mut().unwrap().prev =
                        Some(tail as *mut DblLinkedListNode<'a, T>);
                } else {
                    list_tail = Some(&mut new_node);
                    list_head = Some(new_node);
                }
            }
        }

        DblLinkedList { head: list_head }
    }
}

pub struct DblLinkedListIntoIterator<'a, T>
where
    T: Clone,
{
    cur_node: Option<&'a Box<DblLinkedListNode<'a, T>>>,
}

impl<'a, T> Iterator for DblLinkedListIntoIterator<'a, T>
where
    T: Clone,
{
    type Item = T;
    fn next(&mut self) -> Option<T> {
        let res = (*self.cur_node.unwrap()).value.clone();
        let next_node: Option<&'a Box<DblLinkedListNode<'a, T>>> =
            (*self.cur_node.unwrap()).as_ref().next.as_ref();
        self.cur_node = next_node.map(|x| &*x);
        Some(res)
    }
}

impl<'a, T> DblLinkedList<'a, T>
where
    T: Clone,
{
    /// Returns a new, empty, DblLinkedList
    fn new() -> Self {
        DblLinkedList { head: None }
    }
}

/// Represents a single Doubly-linked List Node
#[derive(Clone, PartialEq, Debug)]
struct DblLinkedListNode<'a, T>
where
    T: Clone,
{
    pub next: Option<Box<DblLinkedListNode<'a, T>>>,
    pub prev: Option<*mut DblLinkedListNode<'a, T>>,
    pub value: T,
}

impl<'a, T> DblLinkedListNode<'a, T>
where
    T: Clone,
{
    /// inserts a new node into the linkedlist after this one
    fn insert_after(&mut self, new_value: &'a T) {
        // construct the new node
        let mut new_node_box: Option<Box<DblLinkedListNode<T>>> =
            Some(Box::from(DblLinkedListNode {
                prev: Some(self),
                next: None,
                value: (*new_value).clone(),
            }));

        // make the new node point to the old next node, then make this node
        // point to the new node,
        std::mem::swap(&mut self.next, &mut new_node_box);
        self.next = new_node_box;
    }
}

/// returns true if all the non-empty blocks in the slice are adjacent
fn is_fragmented<'a, T>(blocks: T) -> bool
where
    T: IntoIterator<Item = Block>,
{
    let mut has_seen_empty: bool = false;
    for block in blocks {
        if block.is_empty {
            has_seen_empty = true;
        } else if !block.is_empty && has_seen_empty {
            return true;
        }
    }

    // if we got here, it isn't fragmented
    false
}

/// "de-frags" a list of blocks, compacting them into the empty space
fn defrag<'a>(blocks: &'a mut DblLinkedList<Block>) {
    while is_fragmented(blocks.into_iter()) {
        /*let last_block_len: &'a mut Block = { blocks.iter_mut().rev().next().unwrap() };
        let first_empty_block: &'a mut Block = blocks.iter_mut().find(|block| block.is_empty).unwrap();

        // if the first empty block is big enough, move into it
        if first_empty_block.len > last_block.len {
            insert_at(
                blocks,
                Block {
                    is_empty: true,
                    id: 0,
                    offset: first_empty_block.offset + last_block.len,
                    len: first_empty_block.len - last_block.len,
                },
                first_empty_idx,
            );
        }*/
    }
}

/// the entrypoint
fn main() {
    // metadata
    let input: String = std::fs::read_to_string("test-input.txt").unwrap();
    println!("n_blocks: {}", input.len());

    // initialize blocks
    let mut cur_is_empty: bool = false;
    let mut cur_offset: usize = 0;
    let mut blocks_vec: Vec<Block> = Vec::new();
    for char in input.trim().chars() {
        let block_len: usize = char.to_digit(10).unwrap() as usize;
        blocks_vec.push(Block {
            is_empty: cur_is_empty,
            id: blocks_vec.len() / 2,
            offset: cur_offset,
            len: block_len,
        });
        cur_is_empty = !cur_is_empty;
        cur_offset += block_len;
    }
    let mut blocks: DblLinkedList<Block> = blocks_vec.into_iter().collect();
    println!("blocks: {:?}", &blocks);
}
