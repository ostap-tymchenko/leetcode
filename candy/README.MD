### Strat 2: edit in place
```mermaid
flowchart TD
s(start) --> A[Is 0?]
A -- No --> B[Larger then neighbor/s?]
B -- No --> J(output: 1)
A -- Yes --> C(output: 1)
B -- a/b --> D[is a/b 0?]
D -- Yes --> F(output: 2)
D -- No --> G(output: a/b+1)
B -- a&b --> E[is a&b 0?]
E -- Yes --> H(output: 2)
E -- No --> I(output: a&b+1)
```

### Strat 3: Layered filter increase strat

[1, 3, 7, 8, 7]
 |  |  |  |  |
 v  v  v  v  v
 1  1  1  1  1 #base
    |  |  |  
    v  v  v
	2  2  2    #has lower neighbor with same number
	   |  |  
       v  v
	   3  3    #has lower neighbor with same number
		  | 
          v
	      4    #has lower neighbor with same number

 1  2  3  4  1 #final
