# Deployment
[![Build Status](https://app.travis-ci.com/shellcodesniper/SimpleDeployer.svg?branch=main)](https://app.travis-ci.com/shellcodesniper/SimpleDeployer)







## Recommend Settings



> apply below json settings to daemon.json
>
> ( logging optimizing )

``` json
{
  "log-driver": "json-file",
  "log-opts": {
    "max-size": "1m",
    "max-file": "1",
    "labels": "kuuwange"
  }
}
```

