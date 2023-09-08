use std::{collections::HashMap, sync::Mutex};
use gloo_console::log;
use derivative::Derivative;
use crate::universe::Cell;

#[derive(Eq, Derivative, Clone)]
#[derivative(PartialEq, Hash)]
pub enum NodePointer {
    Node(Box<Node>),
    Cell(Box<Cell>),
}

// nw: northwest node
// ne: northeast node
// sw: southwest node
// se: southeast node
// nn: north auxiliary node
// ee: east auxiliary node
// ss: south auxiliary node
// ww: west auxiliary node
// cc: central auxiliary node
#[derive(Eq, Derivative, Clone)]
#[derivative(PartialEq, Hash)]
pub struct Node {
    nw: NodePointer,
    ne: NodePointer,
    sw: NodePointer,
    se: NodePointer,
    nn: NodePointer,
    ee: NodePointer,
    ss: NodePointer,
    ww: NodePointer,
    cc: NodePointer,
    depth: usize,
    area: usize,
}

lazy_static! {
    static ref HASHLIFE: Mutex<HashMap<Node, Vec<Cell>>> = {
        let m = HashMap::new();
        Mutex::new(m)
    };
}

impl Default for Node {
    fn default() -> Self {
        Node {
            nw: NodePointer::Cell(Box::new(Cell::Dead)),
            ne: NodePointer::Cell(Box::new(Cell::Dead)),
            sw: NodePointer::Cell(Box::new(Cell::Dead)),
            se: NodePointer::Cell(Box::new(Cell::Dead)),
            nn: NodePointer::Cell(Box::new(Cell::Dead)),
            ee: NodePointer::Cell(Box::new(Cell::Dead)),
            ss: NodePointer::Cell(Box::new(Cell::Dead)),
            ww: NodePointer::Cell(Box::new(Cell::Dead)),
            cc: NodePointer::Cell(Box::new(Cell::Dead)),
            depth: 0,
            area: 0,
        }
    }
}

impl Node {
    pub fn new(cells: Vec<Cell>) -> Self {
        let length = cells.len();
        if (length as f32).sqrt() as usize % 1 != 0 {
            panic!("Not a power of two");
        } 

        let depth = (length as f32).sqrt() as usize;
        if depth == 2 {
            Node {
                nw: NodePointer::Cell(Box::new(cells[0])),
                ne: NodePointer::Cell(Box::new(cells[1])),
                sw: NodePointer::Cell(Box::new(cells[2])),
                se: NodePointer::Cell(Box::new(cells[3])),
                nn: NodePointer::Cell(Box::new(Cell::Dead)),
                ee: NodePointer::Cell(Box::new(Cell::Dead)),
                ss: NodePointer::Cell(Box::new(Cell::Dead)),
                ww: NodePointer::Cell(Box::new(Cell::Dead)),
                cc: NodePointer::Cell(Box::new(Cell::Dead)),
                depth: 2,
                area: 4,
            }
        } else {
            let mut nw_vec: Vec<Cell> = Vec::new();
            for i in 0..(depth / 2) {
                for j in 0..(depth / 2) {
                    nw_vec.push(cells[j + i * depth]);
                }
            }

            let mut ne_vec: Vec<Cell> = Vec::new();
            for i in 0..(depth / 2) {
                for j in (depth / 2)..depth {
                    ne_vec.push(cells[j + i * depth]);
                }
            }

            let mut sw_vec: Vec<Cell> = Vec::new();
            for i in (depth / 2)..depth {
                for j in 0..(depth / 2) {
                    sw_vec.push(cells[j + i * depth]);
                }
            }

            let mut se_vec: Vec<Cell> = Vec::new();
            for i in (depth / 2)..depth {
                for j in (depth / 2)..depth {
                    se_vec.push(cells[j + i * depth]);
                }
            }

            let mut nn_vec: Vec<Cell> = Vec::new();
            for i in 0..(depth / 2) {
                for j in (depth / 4)..(3 * depth / 4) {
                    nn_vec.push(cells[j + i * depth]);
                }
            }

            let mut ee_vec: Vec<Cell> = Vec::new();
            for i in (depth / 4)..(3 * depth / 4) {
                for j in (depth / 2)..depth {
                    ee_vec.push(cells[j + i * depth]);
                }
            }

            let mut ss_vec: Vec<Cell> = Vec::new();
            for i in (depth / 2)..depth {
                for j in (depth / 4)..(3 * depth / 4) {
                    ss_vec.push(cells[j + i * depth]);
                }
            }

            let mut ww_vec: Vec<Cell> = Vec::new();
            for i in (depth / 4)..(3 * depth / 4) {
                for j in 0..(depth / 2) {
                    ww_vec.push(cells[j + i * depth]);
                }
            }

            let mut cc_vec: Vec<Cell> = Vec::new();
            for i in (depth / 4)..(3 * depth / 4) {
                for j in (depth / 4)..(3 * depth / 4) {
                    cc_vec.push(cells[j + i * depth]);
                }
            }

            // let test_string = format!("{:?}", cells);
            // 
            // log!(test_string);

            Node {
                nw: NodePointer::Node(Box::new(Node::new(nw_vec))),
                ne: NodePointer::Node(Box::new(Node::new(ne_vec))),
                sw: NodePointer::Node(Box::new(Node::new(sw_vec))),
                se: NodePointer::Node(Box::new(Node::new(se_vec))),
                nn: NodePointer::Node(Box::new(Node::new(nn_vec))),                 
                ee: NodePointer::Node(Box::new(Node::new(ee_vec))),
                ss: NodePointer::Node(Box::new(Node::new(ss_vec))),
                ww: NodePointer::Node(Box::new(Node::new(ww_vec))),
                cc: NodePointer::Node(Box::new(Node::new(cc_vec))),
                depth,
                area: depth * depth,
            }
        }
    }

    pub fn evolve(&self) -> Vec<Cell> {
        {
            let mut map = HASHLIFE.lock().unwrap();
            let result = map.get(self).cloned().unwrap_or(vec![]);
            if result.len() != 0 { return result; }
        }
        if self.depth == 4 {
            let mut cell_values: Vec<Cell> = vec![Cell::Dead; 16];
            let nodes: Vec<&NodePointer> = vec![&self.nw, &self.ne, &self.se, &self.sw];
            let mut index = 0;
            for j in nodes {
                let mut i: &Box<Node> = &Box::new(Node::default());
                if let NodePointer::Node(k) = j { i = k; }
                let start = match index {
                    0 => 0,
                    1 => 2,
                    2 => 10,
                    3 => 8,
                    _ => 0,
                };
                let mut nw_value = &Box::new(Cell::Dead);
                let mut ne_value = &Box::new(Cell::Dead);
                let mut se_value = &Box::new(Cell::Dead);
                let mut sw_value = &Box::new(Cell::Dead);
                if let NodePointer::Cell(node) = &i.nw { nw_value = node; } 
                if let NodePointer::Cell(node) = &i.ne { ne_value = node; }
                if let NodePointer::Cell(node) = &i.se { se_value = node; }
                if let NodePointer::Cell(node) = &i.sw { sw_value = node; }
                let test_string = format!("{:?}, {:?}, {:?}, {:?}, index: {}", nw_value, ne_value, se_value, sw_value, index);
                log!(test_string);
                cell_values[start] = *nw_value.clone();
                cell_values[start + 1] = *ne_value.clone();
                cell_values[start + 4] = *sw_value.clone();
                cell_values[start + 5] = *se_value.clone();
                index += 1;
            }
            log!("break");
            let test = format!("{:?}", cell_values);
            log!(test);

            let nw_neighbours = [0, 1, 2, 4, 6, 8, 9, 10];
            let mut nw_count = 0;
            for i in nw_neighbours {
                if cell_values[i] == Cell::Alive { nw_count += 1; }
            }

            let ne_neighbours = [1, 2, 3, 5, 7, 9, 10, 11];
            let mut ne_count = 0;
            for i in ne_neighbours {
                if cell_values[i] == Cell::Alive { ne_count += 1; }
            }

            let se_neighbours = [5, 6, 7, 9, 11, 13, 14, 15];
            let mut se_count = 0;
            for i in se_neighbours {
                if cell_values[i] == Cell::Alive { se_count += 1; }
            }

            let sw_neighbours = [4, 5, 6, 8, 10, 12, 13, 14];
            let mut sw_count = 0;
            for i in sw_neighbours {
                if cell_values[i] == Cell::Alive { sw_count += 1; }
            }

            let mut temp_cells: Vec<Cell> = Vec::new();
            let mut check_cells: Vec<Cell> = Vec::new();
            for i in cell_values {
                temp_cells.push(i);
                check_cells.push(i);
            }

            let test_nodes = [[5, nw_count], [6, ne_count], [10, se_count], [9, sw_count]];
            for i in test_nodes {
                if check_cells[i[0]] == Cell::Dead && i[1] == 3 {
                    temp_cells[i[0]] = Cell::Alive;
                } else if check_cells[i[0]] == Cell::Alive && (i[1] == 2 || i[1] == 3) {
                    temp_cells[i[0]] = Cell::Alive;
                } else {
                    temp_cells[i[0]] = Cell::Dead;
                }
            }
            {
                let mut map = HASHLIFE.lock().unwrap();
                map.insert(self.clone(), temp_cells.clone());
            }
            return temp_cells;
        } else {
            let length = (self.area as f32).sqrt() as usize;
            let mut nw_vec: Vec<Cell> = Vec::new();
            let mut ne_vec: Vec<Cell> = Vec::new();
            let mut se_vec: Vec<Cell> = Vec::new();
            let mut sw_vec: Vec<Cell> = Vec::new();
            let mut nn_vec: Vec<Cell> = Vec::new();
            let mut ee_vec: Vec<Cell> = Vec::new();
            let mut ss_vec: Vec<Cell> = Vec::new();
            let mut ww_vec: Vec<Cell> = Vec::new();
            let mut cc_vec: Vec<Cell> = Vec::new();
            if let NodePointer::Node(node) = &self.nw { nw_vec = node.evolve(); }
            if let NodePointer::Node(node) = &self.ne { ne_vec = node.evolve(); }
            if let NodePointer::Node(node) = &self.se { se_vec = node.evolve(); }
            if let NodePointer::Node(node) = &self.sw { sw_vec = node.evolve(); }
            if let NodePointer::Node(node) = &self.nn { nn_vec = node.evolve(); }
            if let NodePointer::Node(node) = &self.ee { ee_vec = node.evolve(); }
            if let NodePointer::Node(node) = &self.ss { ss_vec = node.evolve(); }
            if let NodePointer::Node(node) = &self.ww { ww_vec = node.evolve(); }
            if let NodePointer::Node(node) = &self.cc { cc_vec = node.evolve(); }

            /*
            let mut inter1_vec: Vec<Cell> = vec![Cell::Dead; self.area];
            for i in (length / 8)..(3 * length / 8) {
                for j in (length / 8)..(3 * length / 8) {
                    inter1_vec[j - length / 8 + (i - length / 8) * length / 2] = nw_vec[j + i * length / 2];
                }
            }
            for i in (length / 8)..(3 * length / 8) {
                for j in (length / 8)..(3 * length / 8) {
                    inter1_vec[j + length / 8 + (i - length / 8) * length / 2] = nn_vec[j + i * length / 2];
                }
            }
            for i in (length / 8)..(3 * length / 8) {
                for j in (length / 8)..(3 * length / 8) {
                    inter1_vec[j + length / 8 + (i + length / 8) * length / 2] = cc_vec[j + i * length / 2];
                }
            }
            for i in (length / 8)..(3 * length / 8) {
                for j in (length / 8)..(3 * length / 8) {
                    inter1_vec[j - length / 8 + (i + length / 8) * length / 2] = ee_vec[j + i * length / 2];
                }
            }
            let mut inter2_vec: Vec<Cell> = vec![Cell::Dead; self.area];
            for i in (length / 8)..(3 * length / 8) {
                for j in (length / 8)..(3 * length / 8) {
                    inter2_vec[j - length / 8 + (i - length / 8) * length / 2] = nn_vec[j + i * length / 2];
                }
            }
            for i in (length / 8)..(3 * length / 8) {
                for j in (length / 8)..(3 * length / 8) {
                    inter2_vec[j + length / 8 + (i - length / 8) * length / 2] = ne_vec[j + i * length / 2];
                }
            }
            for i in (length / 8)..(3 * length / 8) {
                for j in (length / 8)..(3 * length / 8) {
                    inter2_vec[j + length / 8 + (i + length / 8) * length / 2] = ee_vec[j + i * length / 2];
                }
            }
            for i in (length / 8)..(3 * length / 8) {
                for j in (length / 8)..(3 * length / 8) {
                    inter2_vec[j - length / 8 + (i + length / 8) * length / 2] = cc_vec[j + i * length / 2];
                }
            }
            let mut inter3_vec: Vec<Cell> = vec![Cell::Dead; self.area];
            for i in (length / 8)..(3 * length / 8) {
                for j in (length / 8)..(3 * length / 8) {
                    inter3_vec[j - length / 8 + (i - length / 8) * length / 2] = cc_vec[j + i * length / 2];
                }
            }
            for i in (length / 8)..(3 * length / 8) {
                for j in (length / 8)..(3 * length / 8) {
                    inter3_vec[j + length / 8 + (i - length / 8) * length / 2] = ee_vec[j + i * length / 2];
                }
            }
            for i in (length / 8)..(3 * length / 8) {
                for j in (length / 8)..(3 * length / 8) {
                    inter3_vec[j + length / 8 + (i + length / 8) * length / 2] = se_vec[j + i * length / 2];
                }
            }
            for i in (length / 8)..(3 * length / 8) {
                for j in (length / 8)..(3 * length / 8) {
                    inter3_vec[j - length / 8 + (i + length / 8) * length / 2] = ss_vec[j + i * length / 2];
                }
            }
            let mut inter4_vec: Vec<Cell> = vec![Cell::Dead; self.area];
            for i in (length / 8)..(3 * length / 8) {
                for j in (length / 8)..(3 * length / 8) {
                    inter4_vec[j - length / 8 + (i - length / 8) * length / 2] = ww_vec[j + i * length / 2];
                }
            }
            for i in (length / 8)..(3 * length / 8) {
                for j in (length / 8)..(3 * length / 8) {
                    inter4_vec[j + length / 8 + (i - length / 8) * length / 2] = cc_vec[j + i * length / 2];
                }
            }
            for i in (length / 8)..(3 * length / 8) {
                for j in (length / 8)..(3 * length / 8) {
                    inter4_vec[j + length / 8 + (i + length / 8) * length / 2] = ss_vec[j + i * length / 2];
                }
            }
            for i in (length / 8)..(3 * length / 8) {
                for j in (length / 8)..(3 * length / 8) {
                    inter4_vec[j - length / 8 + (i + length / 8) * length / 2] = sw_vec[j + i * length / 2];
                }
            }
            

            let inter1_node = Node::new(inter1_vec);
            let inter2_node = Node::new(inter2_vec);
            let inter3_node = Node::new(inter3_vec);
            let inter4_node = Node::new(inter4_vec);
            let inter1_res = inter1_node.evolve().clone();
            let inter2_res = inter2_node.evolve().clone();
            let inter3_res = inter3_node.evolve().clone();
            let inter4_res = inter4_node.evolve().clone();

            let mut res_values: Vec<Cell> = vec![Cell::Dead; self.area];
            for i in (length / 8)..(3 * length / 8) {
                for j in (length / 8)..(3 * length / 8) {
                    res_values[j - length / 8 + (i - length / 8) * length / 2] = inter1_res[j + i * length / 2];
                }
            }
            for i in (length / 8)..(3 * length / 8) {
                for j in (length / 8)..(3 * length / 8) {
                    res_values[j + length / 8 + (i - length / 8) * length / 2] = inter2_res[j + i * length / 2];
                }
            }
            for i in (length / 8)..(3 * length / 8) {
                for j in (length / 8)..(3 * length / 8) {
                    res_values[j + length / 8 + (i + length / 8) * length / 2] = inter3_res[j + i * length / 2];
                }
            }
            for i in (length / 8)..(3 * length / 8) {
                for j in (length / 8)..(3 * length / 8) {
                    res_values[j - length / 8 + (i + length / 8) * length / 2] = inter4_res[j + i * length / 2];
                }
            }
            */


            let mut res_values: Vec<Cell> = vec![Cell::Dead; self.area];
            for i in 0..(3 * length / 8) {
                for j in 0..(3 * length / 8) {
                    res_values[i + j * length] = nw_vec[i + j * length / 2];
                }
            }

            for i in (3 * length / 8)..(5 * length / 8) {
                for j in 0..(3 * length / 8) {
                    res_values[i + j * length] = nn_vec[i - 2 * length / 8 + j * length / 2];
                }
            }

            for i in (5 * length / 8)..length {
                for j in 0..(3 * length / 8) {
                    res_values[i + j * length] = ne_vec[i - 4 * length / 8 + j * length / 2];
                }
            }

            for i in (5 * length / 8)..length {
                for j in (3 * length / 8)..(5 * length / 8) {
                    res_values[i + j * length] = ee_vec[i - 4 * length / 8 + (j - 2 * length / 8) * length / 2];
                }
            }

            for i in (5 * length / 8)..length {
                for j in (5 * length / 8)..length {
                    res_values[i + j * length] = se_vec[i - 4 * length / 8 + (j - 4 * length / 8) * length / 2];
                }
            }

            for i in (3 * length / 8)..(5 * length / 8) {
                for j in (5 * length / 8)..length {
                    res_values[i + j * length] = ss_vec[i - 2 * length / 8 + (j - 4 * length / 8) * length / 2];
                }
            }

            for i in 0..(3 * length / 8) {
                for j in (5 * length / 8)..length {
                    res_values[i + j * length] = sw_vec[i + (j - 4 * length / 8) * length / 2];
                }
            }

            for i in 0..(3 * length / 8) {
                for j in (3 * length / 8)..(5 * length / 8) {
                    res_values[i + j * length] = ww_vec[i + (j - 2 * length / 8) * length / 2];
                }
            }

            for i in (3 * length / 8)..(5 * length / 8) {
                for j in (3 * length / 8)..(5 * length / 8) {
                    res_values[i + j * length] = cc_vec[i - 2 * length / 8 + (j - 2 * length / 8) * length / 2];
                }
            }


            {
                let mut map = HASHLIFE.lock().unwrap();
                map.insert(self.clone(), res_values.clone());
            }
            return res_values; 
        }
    }
}

