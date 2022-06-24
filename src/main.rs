#![crate_type = "staticlib"]
mod buyer;
mod seller;
use crate::buyer::Buyer;
use crate::seller::Seller;
use rand::prelude::*;
use std::{thread, time};
fn main()
  {   
      creation(); //first time loop create sellers and buyers
      fn creation()
      {
        //declaring arrays
        let mut sellers = vec![];
        let mut buyers = vec![];
        for i in 0 .. 6
        {
          let mut rng = thread_rng();
          //mutation cost is the minselling point of the seller. 
          let mutationcost:i32= rng.gen_range(1..100);
          //mutation profit is the current selling point of the seller. 
          let mutationprofit:i32= rng.gen_range(1..100);
          let mut seller= Seller::new(mutationcost, mutationprofit, 's', 32);
          cost(&mut seller);
         //Pushes seller onto the array. 
          sellers.push(seller);
              //Maximum buy of the buyer. 
          let mut mutationmaxbuy:i32= rng.gen_range(mutationcost..100);
          
          //Current willling price of the buyer. 
            let mut mutationprice:i32= rng.gen_range(1..mutationmaxbuy);
          
            //Makes new buyer. 
            let mut buyer= Buyer::new(0, mutationmaxbuy, mutationprice, 's', 32);
            buyers.push(buyer);
  
            
            
           
           
            
          //when the last of the buyers and sellers have been created start pairing them up.
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
       

       // gets all the sellers. 
        for i in 0.. sellers.len()  {
          //random
          let mut rng = thread_rng();
          //using clone function. The reason I am using clone function is because rust is very very memory safe as you cannot afer copying a variable then initialise the variable that was copied. 
          let mut notpairedbuyers = buyers.clone(); 
          let mut m: usize= rng.gen_range(0..notpairedbuyers.len() - 1);
          //remove the buyer that was paired. 
          notpairedbuyers.remove(m);
          //start the individual buyer and seller trading. 
          trading(&mut sellers[i], &mut notpairedbuyers[m]);
        
          //After all the sellers have done trading start the initalising process again. 
          if i == sellers.len() - 1
          {
            //wait again, so I can analyse the data. 
            thread::sleep(time::Duration::from_secs(1));
            //recursion of function with same sellers and buyers. 
            starttrading(sellers, buyers)
          }

        }
         
         


      }
     
      fn trading(seller: &mut Seller, buyer:&mut Buyer)
      {
        //Cleanup code later as I accidentally repeated functions. 
       
        buyer.decidebuy(seller.minsellingpoint, seller);
       
      }
       
//sellers, minselling point. 
fn cost(thing: &mut Seller){

  println!("{}", thing.minsellingpoint);
}


