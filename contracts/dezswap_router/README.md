# Dezswap Router <!-- omit in toc -->

The Router Contract contains the logic to facilitate multi-hop swap operations via dezswap.

**Only Dezswap is supported.**

dimension_37-1 Contract:
- 

cube_47-5 Contract: 
- https://explorer.xpla.io/testnet/address/xpla1pr40depxf8w50y58swdyhc0s2yjptd2xtqgnyfvkz6k40ng53gqqnyftkm

Tx: 
- 

### Operations Assertion
The contract will check whether the resulting token is swapped into one token.

### Example

Swap 
```
{
   "execute_swap_operations":{
      "operations":[
         {
            "dez_swap":{
               "offer_asset_info":{
                  "native_token":{
                     "denom":"uluna"
                  }
               },
               "ask_asset_info":{
                  "token":{
                     "contract_addr":"xpla1cl0kw9axzpzkw58snj6cy0hfp0xp8xh9tudpw2exvzuupn3fafwqqhjc24"
                  }
               }
            }
         },
         {
            "dez_swap":{
               "offer_asset_info":{
                  "token":{
                     "contract_addr":"xpla1cl0kw9axzpzkw58snj6cy0hfp0xp8xh9tudpw2exvzuupn3fafwqqhjc24"
                  }
               },
               "ask_asset_info":{
                  "token":{
                     "contract_addr":"xpla1qnypzwqa03h8vqs0sxjp8hxw0xy5zfwyax26jgnl5k4lw92tjw0scdkrzm"
                  }
               }
            }
         }
      ],
      "minimum_receive":"1"
   }
}
```