.PHONY: get_vehicles
get_vehicles:
    curl http://localhost:7878/vehicles | jq

.PHONY: get_vehicles_query
get_vehicles:
    curl http://localhost:7878/vehicles?year=2025 | jq

.PHONY: post_vehicles
post_vehicles:
    curl -X POST http://localhost:7878/vehicles \
    -H "Content-Type: application/json" \
    -d '{"manufacturer": "Dodge", "model_name": "RAM32", "year": 2021}' | jq
