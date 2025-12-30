# Application Topology

## Source Tree

```sh
├── target                           # Compiled files (sometimes named 'dist' or 'bin')
├── docs                            # Project documentation
│   ├── CONTEXT.md      
│   └── APPLICATION_TOPOLOGY.md
├── src                             # Actual source code files
│   ├── routers
│   │   └── identities.rs     
│   ├── handlers
│   │   └── identities.rs  
│   ├── handlers.rs                     
│   ├── routers.rs                     
│   └── main.rs                     
├── tests                           # Automated tests
├── tools                           # Utility scripts or build tools
├── LICENSE                         # License file
└── README.md                       # Project overview and instructions
```

## Bounded Context

```mermaid
block
    columns 2
    block:presentation
        handlers
        routers
        schemas
    end
    label_presentation["Presentation"]
    space
    blockArrowIdPresesentationApplication<["&nbsp;&nbsp;&nbsp;"]>(y)
    block:application
        routers --> usecases
        ports
    end
    label_application["Application"]
    space
    blockArrowIdApplicationDomain<["&nbsp;&nbsp;&nbsp;"]>(y)
    block:domain
        usecases --> models
    end
    label_domain["Domain"]
    space
    blockArrowIdDomainInfrastructure<["&nbsp;&nbsp;&nbsp;"]>(y)
    block:infrastructure
        models --> repos
        dbs
    end
    label_infrastructure["Infrastructure"]
```
