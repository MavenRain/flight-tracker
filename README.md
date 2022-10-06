# Flight Tracker

This microservice takes a collection of flights (pairs of airport codes) and puts them into canonical form by collapsing connecting flights together.

---

### Calculate

Endpoint: `/calculate`

Sample request:

```
[["IND", "EWR"], ["SFO", "ATL"], ["GSO", "IND"], ["ATL", "GSO"]]
```

Sample response:

```
[["EWR","SFO"]]
```