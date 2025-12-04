import pandas as pd
import matplotlib.pyplot as plt

df = pd.read_excel("loadtest_data.xlsx")

# Separate by scenario
read_df = df[df["scenario"] == "readheavy"]
write_df = df[df["scenario"] == "writeheavy"]

# Separate SQLite and Postgres for convenience
def split_db(data):
    sqlite = data[data["db_type"] == "sqlite"]
    pg = data[data["db_type"] == "postgres"]
    return sqlite, pg

read_sqlite, read_pg = split_db(read_df)
write_sqlite, write_pg = split_db(write_df)

# ----- CREATE SUBPLOTS -----
fig, axes = plt.subplots(2, 2, figsize=(16, 10))
(ax1, ax2), (ax3, ax4) = axes

# ===== Subplot 1: Read-heavy Requests/sec =====
ax1.plot(read_sqlite["users"], read_sqlite["requests_per_sec"], marker="o", label="SQLite")
ax1.plot(read_pg["users"], read_pg["requests_per_sec"], marker="o", label="Postgres")
ax1.set_title("Requests/sec vs Users (Read-Heavy)")
ax1.set_xlabel("Concurrent Users")
ax1.set_ylabel("Requests/sec")
ax1.grid(True, alpha=0.3)
ax1.legend()

# ===== Subplot 2: Read-heavy Latency =====
ax2.plot(read_sqlite["users"], read_sqlite["avg_latency_ms"], marker="o", label="SQLite")
ax2.plot(read_pg["users"], read_pg["avg_latency_ms"], marker="o", label="Postgres")
ax2.set_title("Average Latency vs Users (Read-Heavy)")
ax2.set_xlabel("Concurrent Users")
ax2.set_ylabel("Avg Latency (ms)")
ax2.grid(True, alpha=0.3)
ax2.legend()

# ===== Subplot 3: Write-heavy Requests/sec =====
ax3.plot(write_sqlite["users"], write_sqlite["requests_per_sec"], marker="o", label="SQLite")
ax3.plot(write_pg["users"], write_pg["requests_per_sec"], marker="o", label="Postgres")
ax3.set_title("Requests/sec vs Users (Write-Heavy)")
ax3.set_xlabel("Concurrent Users")
ax3.set_ylabel("Requests/sec")
ax3.grid(True, alpha=0.3)
ax3.legend()

# ===== Subplot 4: Write-heavy Latency =====
ax4.plot(write_sqlite["users"], write_sqlite["avg_latency_ms"], marker="o", label="SQLite")
ax4.plot(write_pg["users"], write_pg["avg_latency_ms"], marker="o", label="Postgres")
ax4.set_title("Average Latency vs Users (Write-Heavy)")
ax4.set_xlabel("Concurrent Users")
ax4.set_ylabel("Avg Latency (ms)")
ax4.grid(True, alpha=0.3)
ax4.legend()

plt.tight_layout()
plt.savefig("combined_subplot.png", dpi=300)
plt.show()
