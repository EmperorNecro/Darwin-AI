use rand::prelude::*;
use std::{thread, time};


#[derive(Copy, Clone)]
struct Seller {
    minsellingpoint:i32,
    profit: i32,
    char: char,
    color: i32
}

impl Seller {
    pub fn new(minsellingpoint: i32, profit: i32, char: char, color:i32) -> Self {
        Seller { minsellingpoint, profit, char, color}
       
    }
    pub fn change(&mut self,sell:bool){
         
      if sell
      {
        self.profit +=1;
      }
      else
      { 
        let comparable = self.profit - 1;
        if comparable == self.minsellingpoint
        {

          return;
        }
        self.profit -=1;
        
      }

      
      
    }




    
}
#[derive(Copy, Clone)]
struct Buyer {
    minbuy: i32,
    maxbuy: i32,
    priceinhead:i32,
    char: char,
    color: i32
}

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
fn main()
  {   
      creation(); //first day create sellers and buyers
      fn creation()
      {
        let mut sellers = vec![];
        let mut buyers = vec![];
        for i in 0 .. 6
        {
          let mut rng = thread_rng();
          
          let mutationcost= rng.gen_range(1..100);
          let mutationprofit= rng.gen_range(1..100);
          let mut seller= Seller::new(mutationcost, mutationprofit, 's', 32);
          cost(&mut seller);
          sellers.push(seller);
          
          let mut mutationmaxbuy= rng.gen_range(mutationcost..100);
          
         
            let mut mutationprice= rng.gen_range(1..mutationmaxbuy);
          
          
            let mut buyer= Buyer::new(0, mutationmaxbuy, mutationprice, 's', 32);
            buyers.push(buyer);
  
            
            
           
           
            
  
            if i == 5
            
            {
              println!("{}", sellers.len());
              println!("{}", buyers.len());
              thread::sleep(time::Duration::from_secs(1));
              starttrading(&mut sellers, &mut buyers);
            
  
            }
        
            
          }
          
          
         
        }
        
      }
     
      //Initalise arrays of sellers and buyers. 
      fn starttrading(sellers: &mut Vec<Seller>, buyers: &mut Vec<Buyer>)
      {
       

       
        for i in 0.. sellers.len()  {
          let mut rng = thread_rng();
          let mut notpairedbuyers = buyers.clone(); 
          let mut m = rng.gen_range(0..notpairedbuyers.len() - 1);
          notpairedbuyers.remove(m);
          trading(&mut sellers[i], &mut notpairedbuyers[m]);
        
          if i == sellers.len() - 1
          {
            thread::sleep(time::Duration::from_secs(1));
            starttrading(sellers, buyers)
          }

        }
         
         


      }
     
      fn trading(seller: &mut Seller, buyer:&mut Buyer)
      {
        //Cleanup code later.
       
        buyer.decidebuy(seller.minsellingpoint, seller);
       
      }
       

fn cost(thing: &mut Seller){

  println!("{}", thing.minsellingpoint);
}
