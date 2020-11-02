project = "expense-api"

app "expense-api" {
  labels = {
    "service" = "expense-api",
    "env"     = "dev"
  }

  build {
    use "docker" {}
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
        port = 8000
    }
  }
}