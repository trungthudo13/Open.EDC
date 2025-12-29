# Application Topology

## Source Tree

```sh
├── build                           # Compiled files (sometimes named 'dist' or 'bin')
├── docs                            # Project documentation
│   ├── CONTEXT.md      
│   └── APPLICATION_TOPOLOGY.md
├── src                             # Actual source code files
│   ├── apis
│   │   └── __init__.py   
│   ├── usecases
│   │   └── __init__.py   
│   ├── ports
│   │   ├── uow.py                  # Unit Of Work
│   │   └── __init__.py  
│   ├── models
│   │   └── __init__.py   
│   ├── services
│   │   └── __init__.py   
│   ├── repos                       # Database bridge, ORM wrapper
│   │   └── __init__.py             
│   ├── schemas                     # Like Dtos
│   │   └── __init__.py             
│   ├── dbs                          
│   │   └── __init__.py   
│   ├── __init__.py
│   └── main.py                     
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
        apis
        schemas
    end
    label_presentation["Presentation"]
    space
    blockArrowIdPresesentationApplication<["&nbsp;&nbsp;&nbsp;"]>(y)
    block:application
        usecases
        ports
    end
    label_application["Application"]
    space
    blockArrowIdApplicationDomain<["&nbsp;&nbsp;&nbsp;"]>(y)
    block:domain
        models
    end
    label_domain["Domain"]
    space
    blockArrowIdDomainInfrastructure<["&nbsp;&nbsp;&nbsp;"]>(y)
    block:infrastructure
        repos
        dbs
    end
    label_infrastructure["Infrastructure"]

    apis --> usecases
    usecases --> models
    models --> repos
```
