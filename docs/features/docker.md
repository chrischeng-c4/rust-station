# Docker Management

The Docker tab provides a visual dashboard for managing development containers.

## Requirements

- Docker Desktop must be installed and running
- Containers should have names prefixed with `rstn-` to be detected

## Service Dashboard

The Docker tab displays:
- **Left panel**: Service cards with status and controls
- **Right panel**: Logs for the selected service

## Service Cards

Each detected service shows:

```
┌────────────────────────────────────────────┐
│ PostgreSQL                    [▶] [↻] [⏹] │
│ postgres:16                                │
│ ● Running   Port: 5432                     │
│                              [+ Add DB]    │
└────────────────────────────────────────────┘
```

| Element | Description |
|---------|-------------|
| **Name** | Service display name |
| **Image** | Docker image tag |
| **Status** | Running (green) / Stopped (gray) |
| **Port** | Exposed port number |
| **Controls** | Start / Restart / Stop |

## Service Controls

| Button | Action |
|--------|--------|
| ▶ Start | `docker start <container>` |
| ↻ Restart | `docker restart <container>` |
| ⏹ Stop | `docker stop <container>` |

## Viewing Logs

Click a service card to view its logs in the right panel:

- **Real-time** tail of container logs
- **Refresh** button to fetch latest
- **Copy** button for clipboard

## Supported Services

| Service | Image | Port | Features |
|---------|-------|------|----------|
| PostgreSQL | `postgres:16` | 5432 | Create DB |
| MySQL | `mysql:8` | 3306 | Create DB |
| MongoDB | `mongo:7` | 27017 | Create DB |
| Redis | `redis:7` | 6379 | - |
| RabbitMQ | `rabbitmq:3-management` | 5672 | Create Vhost |
| NATS | `nats:latest` | 4222 | - |

## Creating Databases

For database services, you can create databases directly:

1. Click **Add DB** on the service card
2. Enter a database name
3. Click **Create Database**
4. Copy the connection string

### Connection String Format

```
PostgreSQL: postgresql://postgres:postgres@localhost:5432/mydb
MySQL:      mysql://root:mysql@localhost:3306/mydb
MongoDB:    mongodb://localhost:27017/mydb
```

## Creating RabbitMQ Vhosts

For RabbitMQ:

1. Click **Add Vhost** on the service card
2. Enter a vhost name
3. Click **Create Vhost**
4. Copy the AMQP URL

## Troubleshooting

### "Docker Not Available"

1. Ensure Docker Desktop is running
2. Try: `docker ps` in terminal
3. Restart Rustation

### Services Not Showing

Rustation detects containers with:
- Name prefix: `rstn-`
- Image matching known service types

To add a custom service, use docker-compose with appropriate naming.
