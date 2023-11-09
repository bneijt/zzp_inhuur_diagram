# zzp_inhuur_diagram
Een diagram van de Nederlandse ZZP inhuur markt

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

```mermaid
flowchart LR
    state_no_zzp["☠ NO ZZP"]
    klant_rabobank["Rabobank"]
    klant_ns["NS"]

    das_headfirst_select((Headfirst Select))
    das_ns(("Inhuur NS"))
    click das_ns "https://inhuur-ns.my.site.com/" "https://inhuur-ns.my.site.com/"

    klant_rabobank --> state_no_zzp
    klant_rabobank --> das_headfirst_select

    klant_ns --> das_ns
```

