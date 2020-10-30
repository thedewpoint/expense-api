project = "expense-api"

app "expense-api" {
  labels = {
    "service" = "expense-api",
    "env"     = "dev"
  }

  build {
    use "pack" {}
    registry {
      use "docker" {
        image = "dewyserver.duckdns.org:80/expense/expense-api"
        tag   = "latest"
      }
    }
  }

  deploy {
    use "kubernetes" {
      probe_path = "/"
    }
  }

  release {
    use "kubernetes" {
    }
  }
}