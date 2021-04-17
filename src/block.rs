#![rustfmt::skip]

use std::collections::BTreeSet;

// 5 x 5 block
#[derive(Debug, Clone, Ord, PartialOrd, PartialEq, Eq)]
pub struct Block{
    initial_value : &'static [i32],
    content: Vec<i32>,
}


impl Block {
    pub fn get_all() -> BTreeSet<Block>{
       vec![Block::new(init::B0), 
            Block::new(init::B1),
            Block::new(init::B2),
            Block::new(init::B3),
            Block::new(init::B4),
            Block::new(init::B5),
            Block::new(init::B6),
            Block::new(init::B7),
            Block::new(init::B8),
            Block::new(init::B9),
            Block::new(init::B10),
            Block::new(init::B11)
        ].into_iter().collect()
    }
    pub fn new(b : &'static [i32])-> Self{
        Block{
            initial_value: b,
            content: b[..].to_vec(),
        }
    }

    pub fn get_content(&self) -> Vec<i32>{
        self.content.clone()
    }

    //block 내 cell 숫자가 5개여야 valid하다
    pub fn is_valid(&self) -> bool{
        self.content.iter().filter(|x|*x > &0).count() == 5
    }

    pub fn reverse(&mut self) {
        for y in 0..5 {
            for x in 0..3 {
                let temp : i32 = *self.get_cell((x,y)).unwrap();
                *self.get_cell((x,y)).unwrap() = *self.get_cell((4-x,y)).unwrap();
                *self.get_cell((4-x,y)).unwrap() = temp;
            }
        }
    }

    pub fn rotate(&mut self) -> &Self{
        let mut org = self.clone();
        for y in 0..5{
            for x in 0..5 {
                *self.get_cell((x, 4-y)).unwrap() = *org.get_cell((y,x)).unwrap();
            }
        }
        self
    }

    pub fn move_left(&mut self)-> &Self{
        let mut zero =0;
        for y in 0..5 {
            for x in 0..5 {
                *self.get_cell((x,y)).unwrap() = *self.get_cell((x+1,y)).unwrap_or(&mut zero);
            }
        }
        self
    }

    pub fn move_right(&mut self)-> &Self{
        let mut zero =0;
        for y in 0..5 {
            for x in (0..5).rev() {
                *self.get_cell((x,y)).unwrap() = *self.get_cell((x-1,y)).unwrap_or(&mut zero);
            }
        }
        self
    }

    pub fn move_up(&mut self)-> &Self{
        let mut zero =0;
        for x in 0..5 {
            for y in 0..5 {
                *self.get_cell((x,y)).unwrap() = *self.get_cell((x,y+1)).unwrap_or(&mut zero);
            }
        }
        self
    }

    pub fn move_down(&mut self)-> &Self{
        let mut zero =0;
        for x in 0..5 {
            for y in (0..5).rev() {
                *self.get_cell((x,y)).unwrap() = *self.get_cell((x,y-1)).unwrap_or(&mut zero);
            }
        }
        self
    }

    pub fn re_init(&mut self) {
       *self = Self::new(self.initial_value) 
    }

    pub fn get_cell_raw(&mut self, idx: usize) -> Option<&mut i32> {
        self.content.get_mut(idx)
    }

    pub fn get_cell(&mut self, (x, y): (i32, i32)) -> Option<&mut i32> {
        if x < 0 || 5 <= x {
            None
        } else if y < 0 || 5 <= y {
            None
        } else {
            Some(&mut self.content[(y * 5+ x) as usize])
        }
    }

    pub fn cell(&self, x: i32, y: i32) -> Option<i32> {
        if x < 0 || 5 <= x {
            None
        } else if y < 0 || 5 <= y {
            None
        } else {
            Some(self.content[(y * 5+ x) as usize])
        }
    }
    
    pub fn nomalize(&mut self) {
        // 최 상단에 값이 존재하게 한다
        while self.content.iter().take(5).filter(|v|**v > 0).count() == 0 {
            self.move_up();
        }

        // 가장 왼쪽에 값이 존재하게 한다
        while self.content.iter().step_by(5).filter(|v|**v > 0).count() == 0 {
            self.move_left();
        }
    }
}


pub mod init{
    pub static B0: &'static[i32] = 
    &[1,1,1,1,1,
      0,0,0,0,0,
      0,0,0,0,0,
      0,0,0,0,0,
      0,0,0,0,0];

    pub static B1: &'static[i32] = 
    &[2,2,2,2,0,
      0,0,2,0,0,
      0,0,0,0,0,
      0,0,0,0,0,
      0,0,0,0,0];

    pub static B2: &'static[i32] = 
    &[3,3,3,0,0,
      0,0,3,3,0,
      0,0,0,0,0,
      0,0,0,0,0,
      0,0,0,0,0];

    pub static B3: &'static[i32] = 
    &[4,4,4,0,0,
      4,0,4,0,0,
      0,0,0,0,0,
      0,0,0,0,0,
      0,0,0,0,0];

    pub static B4: &'static[i32] = 
    &[5,5,5,0,0,
      0,0,5,0,0,
      0,0,5,0,0,
      0,0,0,0,0,
      0,0,0,0,0];

    pub static B5: &'static[i32] = 
    &[6,6,6,6,0,
      0,0,0,6,0,
      0,0,0,0,0,
      0,0,0,0,0,
      0,0,0,0,0];


    pub static B6: &'static[i32] = 
    &[7,7,0,0,0,
      0,7,0,0,0,
      0,7,7,0,0,
      0,0,0,0,0,
      0,0,0,0,0];

    pub static B7: &'static[i32] = 
    &[0,8,0,0,0,
      8,8,8,0,0,
      0,8,0,0,0,
      0,0,0,0,0,
      0,0,0,0,0];

    pub static B8: &'static[i32] = 
    &[0,0,9,0,0,
      9,9,9,0,0,
      0,9,0,0,0,
      0,0,0,0,0,
      0,0,0,0,0];


    pub static B9: &'static[i32] = 
    &[10,0,0,0,0,
      10,10,10,0,0,
      10,0,0,0,0,
      0,0,0,0,0,
      0,0,0,0,0];


    pub static B10: &'static[i32] = 
    &[11,11,0,0,0,
      11,11,0,0,0,
      0 ,11,0,0,0,
      0,0,0,0,0,
      0,0,0,0,0];


    pub static B11: &'static[i32] = 
    &[0, 12,12,0,0,
      12,12,0,0,0,
      12,0,0,0,0,
      0,0,0,0,0,
      0,0,0,0,0];
}



#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn move_left_test(){
        let mut b = Block::new(init::B0);
        assert!(b.is_valid());

        b.move_left(); 
        assert_eq!(b.is_valid(), false);
        let contents = b.get_content();
        assert_eq!(contents.iter().filter(|v|**v > 0 ).count(), 4);
        assert_eq!(contents.iter().take(4).collect::<Vec::<_>>(), vec![&1,&1,&1,&1]);
    }

    #[test]
    fn move_right_test(){
        let mut b = Block::new(init::B0);
        assert!(b.is_valid());

        b.move_right(); 
        assert_eq!(b.is_valid(), false);
        let contents = b.get_content();
        assert_eq!(contents.iter().filter(|v|**v > 0 ).count(), 4);
        assert_eq!(contents.iter().take(5).collect::<Vec::<_>>(), vec![&0, &1,&1,&1,&1]);
    }

    #[test]
    fn move_up_test(){
        let mut b = Block::new(init::B0);
        assert!(b.is_valid());

        b.move_up(); 
        assert_eq!(b.is_valid(), false);
        let contents = b.get_content();
        assert_eq!(contents.iter().filter(|v|**v > 0 ).count(), 0);
    }

    #[test]
    fn move_down_test(){
        let mut b = Block::new(init::B0);
        assert!(b.is_valid());

        b.move_down(); 
        assert_eq!(b.is_valid(), true);
        let contents = b.get_content();
        assert_eq!(contents.iter().filter(|v|**v > 0 ).count(), 5);
        assert_eq!(contents.iter().skip(5).take(5).collect::<Vec::<_>>(), vec![&1,&1,&1,&1,&1]);
    }

    #[test]
    fn reverse_test(){
        let mut b = Block::new(init::B5);
        assert!(b.is_valid());
        b.reverse();
        assert!(b.is_valid());

        assert_eq!(vec![0,6,6,6,6,0,6,0,0,0], b.get_content().into_iter().take(10).collect::<Vec::<_>>());

    }

    #[test]
    fn rotate_nomalize_test(){
        let mut b = Block::new(init::B0);
        b.rotate();
        b.rotate();
        {
            let mut bc = b.clone();
            assert_eq!(bc.get_content().into_iter().collect::<Vec::<_>>(), vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1]);
            bc.nomalize();
            assert_eq!(bc.get_content().into_iter().collect::<Vec::<_>>(), vec![1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        }
        b.rotate();
        assert_eq!(b.get_content().into_iter().collect::<Vec::<_>>(), vec![0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1]);
        b.nomalize(); 
        assert_eq!(b.get_content().into_iter().collect::<Vec::<_>>(), vec![1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0]);

    }
}
