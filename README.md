# data-import-pdf

Script to import in database a list of saga using PDF created by Clator using from the Netophonix forum here the [link](https://forum.netophonix.com/ftopic17700.html) containing all PDF files

## Install

Copy the ```env.dist``` in a ```.env``` file and fill the var with your project information
To find you user UID and GID type in your terminal : ```id```

1 - Build container

```make build```

2 - Start container

```make start```

3 - Get into the container

``make init``

## Diesel 

### Install the diesel cli

```cargo install diesel_cli```

### Run migration 

```diesel migration run```

## Launch the script

Run the script
```cargo run```