#![rustfmt::skip]


// 5 x 5 block
#[derive(Debug, Clone)]
pub struct Block{
    initial_value : &'static [i32],
    content: Vec<i32>,
    pub rotate_count: u8, // 0..4 까지 값이 올 수 있다. 몇번 rotate되었는지 확인하기 위한 용도
    pub reversed : bool, // 좌우 반전 여부
    pub width_move_count : i8 , // 가로 평행이동 숫자 -4 <= x <= 4
    pub height_move_count : i8, // 세로 평행이동 숫자 -4 <= x <- 4
}

impl Block {
    pub fn new(b : &'static [i32])-> Self{
        Block{
            initial_value: b,
            content: b[..].to_vec(),
            rotate_count :0,
            reversed: false,
            width_move_count: 0,
            height_move_count: 0,
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
        self.reversed = !self.reversed;
        for y in 0..5 {
            for x in 0..3 {
                let temp : i32 = *self.get_cell((x,y)).unwrap();
                *self.get_cell((x,y)).unwrap() = *self.get_cell((4-x,y)).unwrap();
                *self.get_cell((4-x,y)).unwrap() = temp;
            }
        }
    }

    pub fn rotate(&mut self) -> &Self{
        self.rotate_count = self.rotate_count + 1;
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
        self.width_move_count += -1;
        for y in 0..5 {
            for x in 0..5 {
                *self.get_cell((x,y)).unwrap() = *self.get_cell((x+1,y)).unwrap_or(&mut zero);
            }
        }
        self
    }

    pub fn move_right(&mut self)-> &Self{
        let mut zero =0;
        self.width_move_count += 1;
        for y in 0..5 {
            for x in (0..5).rev() {
                *self.get_cell((x,y)).unwrap() = *self.get_cell((x-1,y)).unwrap_or(&mut zero);
            }
        }
        self
    }

    pub fn move_up(&mut self)-> &Self{
        let mut zero =0;
        self.height_move_count += -1;
        for x in 0..5 {
            for y in 0..5 {
                *self.get_cell((x,y)).unwrap() = *self.get_cell((x,y+1)).unwrap_or(&mut zero);
            }
        }
        self
    }

    pub fn move_down(&mut self)-> &Self{
        let mut zero =0;
        self.height_move_count += 1;
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
    fn rotate_test(){
        let mut b = Block::new(init::B0);
        b.rotate();
        b.rotate();
        b.rotate();
        assert_eq!(3, b.rotate_count);
        assert_eq!(b.get_content().into_iter().collect::<Vec::<_>>(), vec![0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1])

    }
}
