- Why does it sometimes not write the CSV?

## Tests
- improperly formatted header row 
- with improperly formatted child rows, partially, all, none
- empty records
- big record (like 10,000) 
- records which would withdrawal negative amounts 
- lots of holds / locks 
- rows with wrong data types
- make sure that records are processed in order and stream doesn't mess it up
- chargeback on account which isn't disputed
- resolve on transaction which isn't disputed
- if input file isn't found