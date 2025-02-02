window.BENCHMARK_DATA = {
  "lastUpdate": 1738490466833,
  "repoUrl": "https://github.com/JohnTitor/rust-phf",
  "entries": {
    "Benchmark": [
      {
        "commit": {
          "author": {
            "name": "JohnTitor",
            "username": "JohnTitor"
          },
          "committer": {
            "name": "JohnTitor",
            "username": "JohnTitor"
          },
          "id": "f78614b1f4eb47382025b17c892016bfdaeecc64",
          "message": "chore: Setup workflow for benchmarks gh-pages",
          "timestamp": "2025-02-02T09:43:45Z",
          "url": "https://github.com/JohnTitor/rust-phf/pull/1/commits/f78614b1f4eb47382025b17c892016bfdaeecc64"
        },
        "date": 1738490466197,
        "tool": "cargo",
        "benches": [
          {
            "name": "map::bench_btreemap_none",
            "value": 24.65,
            "range": "± 1.35",
            "unit": "ns/iter"
          },
          {
            "name": "map::bench_btreemap_some",
            "value": 30.71,
            "range": "± 1.55",
            "unit": "ns/iter"
          },
          {
            "name": "map::bench_hashmap_none",
            "value": 15.63,
            "range": "± 0.99",
            "unit": "ns/iter"
          },
          {
            "name": "map::bench_hashmap_some",
            "value": 17.97,
            "range": "± 0.25",
            "unit": "ns/iter"
          },
          {
            "name": "map::bench_match_none",
            "value": 0.31,
            "range": "± 0.02",
            "unit": "ns/iter"
          },
          {
            "name": "map::bench_match_some",
            "value": 0.31,
            "range": "± 0.01",
            "unit": "ns/iter"
          },
          {
            "name": "map::bench_phf_none",
            "value": 16.61,
            "range": "± 0.46",
            "unit": "ns/iter"
          },
          {
            "name": "map::bench_phf_some",
            "value": 21.15,
            "range": "± 0.14",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}