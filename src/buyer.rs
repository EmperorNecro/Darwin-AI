use super::seller::Seller;
#[derive(Copy, Clone)]
//Buyer class minbuy (not used, will clean up code of this in the future). Maxbuy the maximum value they can buy. 
pub struct Buyer {
    minbuy: i32,
    maxbuy: i32,
    priceinhead:i32,
    char: char,
    color: i32
}
//Buyer functions: new is the same as before. Decide buy is the decision making phase. 
impl Buyer {
    pub fn new(minbuy: i32, maxbuy: i32, priceinhead: i32, char: char, color:i32) -> Self {
        Buyer { minbuy, maxbuy,priceinhead, char, color}
       
    }
    
    pub fn decidebuy(&mut self, cost:i32 , seller:&mut Seller){
      //buyers "more than price" less than cost. 
      if self.priceinhead +5 < cost
      {
        let comparable = self.priceinhead + 1;
        if comparable == self.priceinhead
        {

          return;
        }
        self.priceinhead += 1;
        println!("failure");
        seller.change(false);
        
        return; 
      }
      //buyers price more than cost. 
      if self.priceinhead > cost 
      {

        self.priceinhead -= 1;
        println!("{} go down", self.priceinhead);
        seller.change(true);

        return;
        //increase priceinhead

      }
      //buyers "more than price" more than cost. 
      if self.priceinhead+5 >= cost
      { 
        let comparable = self.priceinhead + 1;
        if comparable == self.priceinhead
        {

          return;
        }
        self.priceinhead += 1;
        
        println!("{} go up", self.priceinhead);
        seller.change(false);

        return;
         //decreasepriceinhead
            
      }
    }


    

}
