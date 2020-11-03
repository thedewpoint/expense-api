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
      service_port = 8000
    }
  }

  release {
    use "kubernetes" {
        load_balancer = true
        port = 8000
    }
  }
}