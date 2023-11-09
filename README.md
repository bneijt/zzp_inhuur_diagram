# zzp_inhuur_diagram

Een diagram van de Nederlandse ZZP inhuur markt

De volgende zijn de basis elementen van het diagram.
```mermaid
flowchart LR
    klant["Klant"]
    broker([Broker])
    portal((Portal/marktplaats))
    preferred[\Preferred supplier/]

    klant --> broker
    klant --> portal
    klant --> preferred
```

Zie [inhuur_diagram.md](inhuur_diagram.md)
