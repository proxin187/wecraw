# Wecraw

Wecraw is a web crawler and local search engine using tf-idf to search.

## Crawling
Before you can search you will need to index a few websites,
this can be done with the following command:
```
$ cargo run --release -- --seed <URL>
```

this will automatically save the indexed pages to model.json when you press
<ESC> to exit the program.

## Hosting
To host the search engine you need to run the following command:
```
$ cargo run --release -- --serve model.json
```



