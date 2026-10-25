import marimo

__generated_with = "0.25.1"
app = marimo.App()


@app.cell(hide_code=True)
def _(mo):
    mo.md(r"""
    # Load Data
    """)
    return


@app.cell(hide_code=True)
def _():
    from pathlib import Path

    import marimo as mo
    import polars as pl

    return Path, mo, pl


@app.cell(hide_code=True)
def _(Path):
    DATA_DIR = Path("data")
    return (DATA_DIR,)


@app.cell(hide_code=True)
def _(DATA_DIR):
    protocol_files = {
        path.stem: path
        for path in sorted(DATA_DIR.glob("*.csv"))
        if path.name != "metadata.csv"
    }
    return (protocol_files,)


@app.cell(hide_code=True)
def _(DATA_DIR, pl):
    metadata = pl.read_csv(DATA_DIR / "metadata.csv")
    return (metadata,)


@app.cell(hide_code=True)
def _(pl, protocol_files):
    ping_raw = pl.read_csv(protocol_files["ping"])
    http_raw = pl.read_csv(
        protocol_files["http"],
        null_values=["timeout"]
    )
    return http_raw, ping_raw


@app.cell(hide_code=True)
def _(mo):
    mo.md(r"""
    # Distance Calculation
    """)
    return


@app.cell(hide_code=True)
def _():
    from geopy.distance import geodesic

    return (geodesic,)


@app.cell(hide_code=True)
def _(metadata, pl):
    node_pairs = (
        metadata
        .select(
            pl.col("location").alias("src"),
            pl.col("lat").alias("src_lat"),
            pl.col("lon").alias("src_lon"),
        )
        .join(
            metadata.select(
                pl.col("location").alias("dst"),
                pl.col("lat").alias("dst_lat"),
                pl.col("lon").alias("dst_lon"),
            ),
            how="cross",
        )
    )
    return (node_pairs,)


@app.cell(hide_code=True)
def _(geodesic, node_pairs, pl):
    distances = node_pairs.with_columns(
        pl.struct(
            ["src_lat", "src_lon", "dst_lat", "dst_lon"]
        )
        .map_elements(
            lambda x: geodesic(
                (x["src_lat"], x["src_lon"]),
                (x["dst_lat"], x["dst_lon"]),
            ).kilometers,
            return_dtype=pl.Float64,
        )
        .alias("distance_km")
    )
    return (distances,)


@app.cell(hide_code=True)
def _(distances, node_order, pl):
    distance_matrix = (
        distances
        .with_columns(
            pl.col("distance_km").round(0).cast(pl.Int64)
        )
        .to_pandas()
        .pivot(
            index="src",
            columns="dst",
            values="distance_km",
        )
        .reindex(
            index=node_order,
            columns=node_order,
        )
    )
    return (distance_matrix,)


@app.cell(hide_code=True)
def _(mo):
    def distance_matrix_to_latex(
        matrix,
        *,
        caption="Distance matrix of locations used in evaluations in km.",
        label="tab:distancematrix",
        resize_width="0.8",
    ):
        locations = list(matrix.columns)
        n_cols = len(locations) + 1

        # Column spec: one left-aligned "Location" column + n right-aligned data columns
        column_spec = "|l|" + "r|" * len(locations)

        # Header row: Location + italic bold location names
        header = (
            r"\textit{\textbf{Location}} & "
            + " & ".join(rf"\textit{{{loc}}}" for loc in locations)
            + r" \\ \hline"
        )

        # Body rows
        body_lines = []
        for src, row in matrix.iterrows():
            cells = []
            for dst in locations:
                value = row[dst]
                cells.append("" if value is None else f"{int(value)}")
            body_lines.append(
                rf"\textit{{{src}}} & " + " & ".join(cells) + r" \\ \hline"
            )
        body = "\n".join(body_lines)

        latex = (
            "\\begin{table*}[ht]\n"
            "\\centering\n"
            f"\\resizebox{{{resize_width}\\textwidth}}{{!}}{{%\n"
            f"\\begin{{tabular}}{{{column_spec}}}\n"
            "\\hline\n"
            f"{header}\n"
            f"{body}\n"
            "\\end{tabular}%\n"
            "}\n"
            f"\\caption{{{caption}}}\n"
            f"\\label{{{label}}}\n"
            "\\end{table*}"
        )

        return mo.md(f"```latex\n{latex}\n```")

    return (distance_matrix_to_latex,)


@app.cell
def _(distance_matrix, distance_matrix_to_latex):
    distance_matrix_to_latex(distance_matrix)
    return


@app.cell(hide_code=True)
def _(mo):
    mo.md(r"""
    # Data Transformation
    """)
    return


@app.cell(hide_code=True)
def _(mo):
    mo.md(r"""
    ## Ping
    """)
    return


@app.cell(hide_code=True)
def _(metadata, ping_raw, pl):
    ping_with_nodes = ping_raw.join(
        metadata.select(
            pl.col("ip").alias("src_addr"),
            pl.col("location").alias("src"),
            pl.col("anchor").alias("src_anchor"),
        ),
        on="src_addr",
        how="left",
    ).join(
        metadata.select(
            pl.col("ip").alias("dst_addr"),
            pl.col("location").alias("dst"),
            pl.col("anchor").alias("dst_anchor"),
        ),
        on="dst_addr",
        how="left",
    )
    return (ping_with_nodes,)


@app.cell(hide_code=True)
def _(distances, ping_with_nodes, pl):
    ping_enriched = ping_with_nodes.join(
        distances.select(["src", "dst", "distance_km"]),
        on=["src", "dst"],
        how="left",
    )
    ping_enriched = ping_enriched.with_columns(
        pl.from_epoch("timestamp", time_unit="s").alias("datetime")
    )
    return (ping_enriched,)


@app.cell(hide_code=True)
def _(mo):
    mo.md(r"""
    ## HTTP
    """)
    return


@app.cell(hide_code=True)
def _(http_raw, metadata, pl):
    http_with_nodes = http_raw.join(
        metadata.select(
            pl.col("ip").alias("src_addr"),
            pl.col("location").alias("src"),
            pl.col("anchor").alias("src_anchor"),
        ),
        on="src_addr",
        how="left",
    ).join(
        metadata.select(
            pl.col("ip").alias("dst_addr"),
            pl.col("location").alias("dst"),
            pl.col("anchor").alias("dst_anchor"),
        ),
        on="dst_addr",
        how="left",
    )
    return (http_with_nodes,)


@app.cell(hide_code=True)
def _(distances, http_with_nodes, pl):
    http_enriched = http_with_nodes.join(
        distances.select(["src", "dst", "distance_km"]),
        on=["src", "dst"],
        how="left",
    )
    http_enriched = http_enriched.with_columns(
        pl.from_epoch("timestamp", time_unit="s").alias("datetime")
    )
    return (http_enriched,)


@app.cell(hide_code=True)
def _(http_failures, http_raw, mo, ping_failures, ping_raw):
    http_attempts = http_raw.height
    lost_packets = int(ping_failures["lost"].sum())
    sent_packets = int(ping_raw["sent"].sum())
    mo.md(
        f"""
        # Failure Analysis

        A measurement counts as failed when

        * **HTTP** reports an `err` for the request — DNS resolution failure,
          connect timeout, or timeout while reading the status line, or
        * **ICMP** receives fewer echo replies than were sent (`rcvd < sent`).

        | Protocol | Attempts | Failures | Rate |
        | --- | ---: | ---: | ---: |
        | HTTP | {http_attempts:,} requests | {http_failures.height:,} | {http_failures.height / http_attempts:.2%} |
        | ICMP | {sent_packets:,} packets | {lost_packets:,} lost | {lost_packets / sent_packets:.2%} |

        ICMP failures hit {ping_failures.height:,} probe runs, of which
        {ping_failures.filter(ping_failures["cause"] == "Total loss").height:,}
        lost every packet.
        """
    )
    return


@app.cell(hide_code=True)
def _(http_enriched, ping_enriched, pl):
    # Every measurement targets exactly one destination. DNS failures carry no
    # destination address, so recover it from the successful requests of the
    # same measurement.
    http_targets = (
        http_enriched
        .filter(pl.col("dst").is_not_null())
        .group_by("msm_id")
        .agg(pl.col("dst").first().alias("target"))
    )
    http_measurements = (
        http_enriched
        .join(http_targets, on="msm_id", how="left")
        .with_columns(pl.coalesce("dst", "target").alias("dst"))
    )

    def classify_http_error(err):
        if err.startswith("connect"):
            return "Connect timeout"
        if "name resolution" in err:
            return "DNS resolution"
        return "Read timeout"

    http_failures = (
        http_measurements
        .filter(pl.col("err").is_not_null())
        .with_columns(
            pl.col("err")
            .map_elements(classify_http_error, return_dtype=pl.String)
            .alias("cause")
        )
    )

    ping_failures = (
        ping_enriched
        .filter(pl.col("rcvd") < pl.col("sent"))
        .with_columns(
            pl.when(pl.col("rcvd") == 0)
            .then(pl.lit("Total loss"))
            .otherwise(pl.lit("Partial loss"))
            .alias("cause"),
            (pl.col("sent") - pl.col("rcvd")).alias("lost"),
        )
    )
    return http_failures, http_measurements, ping_failures


@app.cell(hide_code=True)
def _(mo):
    mo.md(r"""
    ## Failures by Destination Anchor

    Grouping the failures by destination shows whether a problem is specific to
    one anchor or spread across the network. HTTP failure counts are split by
    cause; ICMP failures are split into runs that lost every packet and runs
    with partial loss.
    """)
    return


@app.cell(hide_code=True)
def _(http_failures, http_measurements, pl):
    http_failure_summary = (
        http_measurements
        .group_by("dst")
        .agg(pl.len().alias("requests"))
        .join(
            http_failures.group_by("dst").agg(
                pl.len().alias("failed"),
                (pl.col("cause") == "DNS resolution").sum().alias("DNS errors"),
                (pl.col("cause") == "Connect timeout").sum().alias("Connect timeouts"),
                (pl.col("cause") == "Read timeout").sum().alias("Read timeouts"),
            ),
            on="dst",
            how="left",
        )
        .with_columns(
            pl.col("failed", "DNS errors", "Connect timeouts", "Read timeouts")
            .fill_null(0)
        )
        .with_columns(
            (pl.col("failed") / pl.col("requests") * 100)
            .round(3)
            .alias("failure_%")
        )
        .sort("failure_%", descending=True)
    )
    http_failure_summary
    return (http_failure_summary,)


@app.cell(hide_code=True)
def _(ping_enriched, ping_failures, pl):
    ping_failure_summary = (
        ping_enriched
        .group_by("dst")
        .agg(
            pl.col("sent").sum().alias("packets"),
            (pl.col("sent") - pl.col("rcvd")).sum().alias("lost_packets"),
        )
        .join(
            ping_failures.group_by("dst").agg(
                pl.len().alias("affected"),
                (pl.col("cause") == "Total loss").sum().alias("Total loss"),
                (pl.col("cause") == "Partial loss").sum().alias("Partial loss"),
            ),
            on="dst",
            how="left",
        )
        .with_columns(
            pl.col("affected", "Total loss", "Partial loss").fill_null(0)
        )
        .with_columns(
            (pl.col("lost_packets") / pl.col("packets") * 100)
            .round(3)
            .alias("loss_%")
        )
        .sort("loss_%", descending=True)
    )
    ping_failure_summary
    return (ping_failure_summary,)


@app.cell(hide_code=True)
def _(http_failure_summary, ping_failure_summary, plt):
    failure_fig, (failure_ax_http, failure_ax_ping) = plt.subplots(1, 2, figsize=(16, 6))

    failure_http_plot = http_failure_summary.sort("failed").to_pandas()
    failure_left = None
    for failure_label, failure_column, failure_color in [
        ("DNS resolution", "DNS errors", "#4C72B0"),
        ("Connect timeout", "Connect timeouts", "#DD8452"),
        ("Read timeout", "Read timeouts", "#C44E52"),
    ]:
        failure_ax_http.barh(
            failure_http_plot["dst"],
            failure_http_plot[failure_column],
            left=failure_left,
            label=failure_label,
            color=failure_color,
        )
        failure_left = (
            failure_http_plot[failure_column]
            if failure_left is None
            else failure_left + failure_http_plot[failure_column]
        )

    for failure_row, (_, failure_values) in enumerate(
        failure_http_plot.iterrows()
    ):
        failure_ax_http.text(
            failure_values["failed"] + 0.6,
            failure_row,
            f"{int(failure_values['failed'])} ({failure_values['failure_%']:.2f}%)",
            va="center",
            fontsize=9,
        )

    failure_ax_http.set_xlim(0, failure_http_plot["failed"].max() * 1.4)
    failure_ax_http.set_xlabel("Failed requests")
    failure_ax_http.set_title("HTTP failures by destination")
    failure_ax_http.legend(loc="lower right", title="Cause", frameon=True)
    failure_ax_http.grid(axis="x", linestyle=":", alpha=0.4)

    failure_ping_plot = ping_failure_summary.sort("affected").to_pandas()
    failure_left = None
    for failure_label, failure_column, failure_color in [
        ("Total loss", "Total loss", "#C44E52"),
        ("Partial loss", "Partial loss", "#DD8452"),
    ]:
        failure_ax_ping.barh(
            failure_ping_plot["dst"],
            failure_ping_plot[failure_column],
            left=failure_left,
            label=failure_label,
            color=failure_color,
        )
        failure_left = (
            failure_ping_plot[failure_column]
            if failure_left is None
            else failure_left + failure_ping_plot[failure_column]
        )

    for failure_row, (_, failure_values) in enumerate(
        failure_ping_plot.iterrows()
    ):
        failure_ax_ping.text(
            failure_values["affected"] + 0.5,
            failure_row,
            f"{int(failure_values['affected'])} ({failure_values['loss_%']:.2f}% loss)",
            va="center",
            fontsize=9,
        )

    failure_ax_ping.set_xlim(0, failure_ping_plot["affected"].max() * 1.5)
    failure_ax_ping.set_xlabel("Affected probe runs")
    failure_ax_ping.set_title("ICMP packet loss by destination")
    failure_ax_ping.legend(loc="lower right", title="Failure", frameon=True)
    failure_ax_ping.grid(axis="x", linestyle=":", alpha=0.4)

    failure_fig.suptitle(
        "Failed measurements per destination anchor", fontsize=16
    )
    plt.tight_layout()

    failure_plot = failure_fig
    failure_plot
    return (failure_plot,)


@app.cell(hide_code=True)
def _(http_failures, mo, ping_failures):
    mo.accordion(
        {
            f"HTTP failures ({http_failures.height})": http_failures.select(
                "datetime", "src", "dst", "cause", "err", "prb_id", "msm_id"
            ),
            f"ICMP failures ({ping_failures.height})": ping_failures.select(
                "datetime", "src", "dst", "cause", "sent", "rcvd", "lost", "prb_id",
                "msm_id",
            ),
        }
    )
    return


@app.cell(hide_code=True)
def _(mo):
    mo.md(r"""
    # Heatmaps
    """)
    return


@app.cell(hide_code=True)
def _():
    import matplotlib.pyplot as plt
    import seaborn as sns

    return plt, sns


@app.cell(hide_code=True)
def _(metadata):
    node_order = metadata["location"].to_list()
    return (node_order,)


@app.cell(hide_code=True)
def _(plt, sns):
    def plot_heatmap(
        data,
        *,
        title=None,
        xlabel="Destination",
        ylabel="Source",
        colorbar_label="Latency (ms)",
        cmap="magma",
        figsize=(10, 8),
        annot=True,
        fmt=".1f",
        annot_fontsize=12,
        tick_fontsize=13,
        axis_label_fontsize=16,
        colorbar_fontsize=14,
        colorbar_tick_fontsize=12,
        title_fontsize=18,
        x_rotation=45,
        y_rotation=0,
    ):
        fig, ax = plt.subplots(figsize=figsize)

        sns.heatmap(
            data,
            annot=annot,
            fmt=fmt,
            cmap=cmap,
            annot_kws={
                "fontsize": annot_fontsize,
            },
            linewidths=1,
            linecolor="white",
            cbar_kws={
                "label": colorbar_label,
            },
            ax=ax,
        )

        ax.set_xlabel(
            xlabel,
            fontsize=axis_label_fontsize,
            labelpad=10,
        )

        ax.set_ylabel(
            ylabel,
            fontsize=axis_label_fontsize,
            labelpad=10,
        )

        ax.tick_params(
            axis="x",
            labelsize=tick_fontsize,
            rotation=x_rotation,
        )

        ax.tick_params(
            axis="y",
            labelsize=tick_fontsize,
            rotation=y_rotation,
        )

        cbar = ax.collections[0].colorbar

        cbar.set_label(
            colorbar_label,
            fontsize=colorbar_fontsize,
        )

        cbar.ax.tick_params(
            labelsize=colorbar_tick_fontsize,
        )

        if title is not None:
            ax.set_title(
                title,
                fontsize=title_fontsize,
                pad=15,
            )

        plt.tight_layout()

        return fig

    return (plot_heatmap,)


@app.cell(hide_code=True)
def _(mo):
    mo.md(r"""
    ## RAW
    """)
    return


@app.cell(hide_code=True)
def _(http_enriched, ping_enriched, pl):
    latency_heatmap_raw_ping = (
        ping_enriched
        .group_by(["src", "dst"])
        .agg(
            pl.col("avg").mean().alias("avg")
        )
    )

    latency_heatmap_raw_http = (
        http_enriched
        .group_by(["src", "dst"])
        .agg(
            pl.col("rt").mean().alias("rt")
        )
    )
    return latency_heatmap_raw_http, latency_heatmap_raw_ping


@app.cell(hide_code=True)
def _(mo):
    mo.md(r"""
    ### Raw Ping Latency
    """)
    return


@app.cell(hide_code=True)
def _(latency_heatmap_raw_ping, node_order, plot_heatmap):
    heatmap_data_ping_raw = (
            latency_heatmap_raw_ping
            .to_pandas()
            .pivot(
                index="src",
                columns="dst",
                values="avg",
            )
            .reindex(
                index=node_order,
                columns=node_order,
            )
        )

    ping_heatmap_raw = plot_heatmap(
        heatmap_data_ping_raw,
        xlabel="Destination",
        ylabel="Source",
        title="Average ICMP Latency",
        colorbar_label="Latency (ms)",
        cmap="magma",
    )

    ping_heatmap_raw
    return (ping_heatmap_raw,)


@app.cell(hide_code=True)
def _(mo):
    mo.md(r"""
    ### Raw HTTP Latency
    """)
    return


@app.cell(hide_code=True)
def _(latency_heatmap_raw_http, node_order, plot_heatmap):
    heatmap_data_http_raw = (
        latency_heatmap_raw_http
        .to_pandas()
        .pivot(
            index="src",
            columns="dst",
            values="rt",
        )
        .reindex(
            index=node_order,
            columns=node_order,
        )
    )

    http_heatmap_raw = plot_heatmap(
        heatmap_data_http_raw,
        xlabel="Destination",
        ylabel="Source",
        title="Average HTTP Latency",
        colorbar_label="Latency (ms)",
        cmap="magma",
    )

    http_heatmap_raw
    return (http_heatmap_raw,)


@app.cell(hide_code=True)
def _(mo):
    mo.md(r"""
    # Distance
    """)
    return


@app.cell(hide_code=True)
def _(ping_enriched, pl):
    distance_latency = (
        ping_enriched
        .group_by(["src", "dst", "distance_km"])
        .agg(
            pl.col("avg").mean().alias("latency_ms")
        )
        .filter(
            pl.col("src") != pl.col("dst")
        )
    )
    return (distance_latency,)


@app.cell(hide_code=True)
def _(distance_latency, plt, sns):
    import numpy as np
    from scipy.stats import linregress    

    plot_data = distance_latency.to_pandas()

    regression = linregress(
        plot_data["distance_km"],
        plot_data["latency_ms"],
    )

    slope = regression.slope
    intercept = regression.intercept
    r_value = regression.rvalue
    p_value = regression.pvalue
    r_squared = r_value**2

    slope_100km = regression.slope * 100

    slope_se_100km = regression.stderr * 100
    ci_low = slope_100km - 1.96 * slope_se_100km
    ci_high = slope_100km + 1.96 * slope_se_100km

    distance_ping_plot, ax_distance_ping = plt.subplots(
        figsize=(10, 7)
    )

    sns.scatterplot(
        data=plot_data,
        x="distance_km",
        y="latency_ms",
        s=70,
        alpha=0.7,
        ax=ax_distance_ping,
    )

    sns.regplot(
        data=plot_data,
        x="distance_km",
        y="latency_ms",
        scatter=False,
        ci=95,
        line_kws={
            "linewidth": 2.5,
        },
        ax=ax_distance_ping,
    )

    x_max = plot_data["distance_km"].max()

    x_theoretical = np.linspace(
        0,
        x_max,
        200,
    )

    y_theoretical = (
        (2 * x_theoretical / 100)
        * 0.4895
    )

    ax_distance_ping.plot(
        x_theoretical,
        y_theoretical,
        linestyle="--",
        linewidth=2,
        label="Theoretical minimum RTT",
    )

    stats_text = (
        f"Trend: {slope_100km:.2f} ms / 100 km\n"
        f"95% CI: [{ci_low:.2f}, {ci_high:.2f}]\n"
        f"Intercept: {intercept:.2f} ms\n"
        f"$R^2$: {r_squared:.3f}\n"
        f"$p$: {p_value:.2e}"
    )

    ax_distance_ping.text(
        0.03,
        0.97,
        stats_text,
        transform=ax_distance_ping.transAxes,
        fontsize=12,
        verticalalignment="top",
        bbox=dict(
            boxstyle="round,pad=0.5",
            facecolor="white",
            alpha=0.9,
        ),
    )

    ax_distance_ping.set_xlabel(
        "Geographic distance (km)",
        fontsize=16,
        labelpad=10,
    )

    ax_distance_ping.set_ylabel(
        "Average RTT latency (ms)",
        fontsize=16,
        labelpad=10,
    )

    ax_distance_ping.tick_params(
        axis="both",
        labelsize=13,
    )

    ax_distance_ping.set_title(
        "RTT latency vs. geographic distance",
        fontsize=18,
        pad=15,
    )

    ax_distance_ping.legend(
        fontsize=12,
        frameon=True,
    )

    ax_distance_ping.grid(
        True,
        alpha=0.25,
    )

    plt.tight_layout()

    distance_ping_plot
    return distance_ping_plot, linregress, np


@app.cell
def _(mo):
    mo.md(r"""
    # HTTP Distance
    """)
    return


@app.cell(hide_code=True)
def _(http_enriched, pl):
    http_distance_latency = (
        http_enriched
        .group_by(["src", "dst", "distance_km"])
        .agg(
            pl.col("rt").mean().alias("latency_ms")
        )
        .filter(
            pl.col("src") != pl.col("dst")
        )
    )
    return (http_distance_latency,)


@app.cell(hide_code=True)
def _(http_distance_latency, linregress, np, plt, sns):
    http_plot_data = http_distance_latency.to_pandas()

    http_regression = linregress(
        http_plot_data["distance_km"],
        http_plot_data["latency_ms"],
    )

    http_slope = http_regression.slope
    http_intercept = http_regression.intercept
    http_r_value = http_regression.rvalue
    http_p_value = http_regression.pvalue
    http_r_squared = http_r_value**2

    http_slope_100km = http_regression.slope * 100

    http_slope_se_100km = http_regression.stderr * 100
    http_ci_low = http_slope_100km - 1.96 * http_slope_se_100km
    http_ci_high = http_slope_100km + 1.96 * http_slope_se_100km

    http_distance_ping_plot, ax_http_distance_ping = plt.subplots(
        figsize=(10, 7)
    )

    sns.scatterplot(
        data=http_plot_data,
        x="distance_km",
        y="latency_ms",
        s=70,
        alpha=0.7,
        ax=ax_http_distance_ping,
    )

    sns.regplot(
        data=http_plot_data,
        x="distance_km",
        y="latency_ms",
        scatter=False,
        ci=95,
        line_kws={
            "linewidth": 2.5,
        },
        ax=ax_http_distance_ping,
    )

    http_x_max = http_plot_data["distance_km"].max()

    http_x_theoretical = np.linspace(
        0,
        http_x_max,
        200,
    )

    http_y_theoretical = (
        (2 * http_x_theoretical / 100)
        * 0.4895
    )

    ax_http_distance_ping.plot(
        http_x_theoretical,
        http_y_theoretical,
        linestyle="--",
        linewidth=2,
        label="Theoretical minimum RTT",
    )

    http_stats_text = (
        f"Trend: {http_slope_100km:.2f} ms / 100 km\n"
        f"95% CI: [{http_ci_low:.2f}, {http_ci_high:.2f}]\n"
        f"Intercept: {http_intercept:.2f} ms\n"
        f"$R^2$: {http_r_squared:.3f}\n"
        f"$p$: {http_p_value:.2e}"
    )

    ax_http_distance_ping.text(
        0.03,
        0.97,
        http_stats_text,
        transform=ax_http_distance_ping.transAxes,
        fontsize=12,
        verticalalignment="top",
        bbox=dict(
            boxstyle="round,pad=0.5",
            facecolor="white",
            alpha=0.9,
        ),
    )

    ax_http_distance_ping.set_xlabel(
        "Geographic distance (km)",
        fontsize=16,
        labelpad=10,
    )

    ax_http_distance_ping.set_ylabel(
        "Average RTT latency (ms)",
        fontsize=16,
        labelpad=10,
    )

    ax_http_distance_ping.tick_params(
        axis="both",
        labelsize=13,
    )

    ax_http_distance_ping.set_title(
        "HTTP RTT latency vs. geographic distance",
        fontsize=18,
        pad=15,
    )

    ax_http_distance_ping.legend(
        fontsize=12,
        frameon=True,
    )

    ax_http_distance_ping.grid(
        True,
        alpha=0.25,
    )

    plt.tight_layout()

    http_distance_ping_plot
    return (http_distance_ping_plot,)


@app.cell(hide_code=True)
def _(mo):
    mo.md(r"""
    # Network Asymetry
    """)
    return


@app.cell(hide_code=True)
def _(ping_enriched, pl):
    directional_latency = (
        ping_enriched
        .group_by(["src", "dst"])
        .agg(
            pl.col("avg").mean().alias("latency_ms")
        )
    )
    return (directional_latency,)


@app.cell(hide_code=True)
def _(directional_latency, pl):
    forward = directional_latency.rename(
        {
            "src": "node_a",
            "dst": "node_b",
            "latency_ms": "latency_ab",
        }
    )

    reverse = directional_latency.rename(
        {
            "src": "node_b",
            "dst": "node_a",
            "latency_ms": "latency_ba",
        }
    )

    asymmetry = (
        forward.join(
            reverse,
            on=["node_a", "node_b"],
            how="inner",
        )
        .filter(pl.col("node_a") < pl.col("node_b"))
        .with_columns(
            (pl.col("latency_ab") - pl.col("latency_ba"))
            .abs()
            .alias("asymmetry_ms")
        )
        .with_columns(
            (
                pl.col("asymmetry_ms")
                / ((pl.col("latency_ab") + pl.col("latency_ba")) / 2)
                * 100
            ).alias("asymmetry_percent")
        )
        .sort("asymmetry_ms", descending=True)
    )
    return (asymmetry,)


@app.cell(hide_code=True)
def _(asymmetry, plt, sns):
    asymmetry_plot, ax = plt.subplots(figsize=(8, 8))

    sns.scatterplot(
        data=asymmetry.to_pandas(),
        x="latency_ab",
        y="latency_ba",
        s=80,
        ax=ax,
    )

    limits = [
        min(
            asymmetry["latency_ab"].min(),
            asymmetry["latency_ba"].min(),
        ),
        max(
            asymmetry["latency_ab"].max(),
            asymmetry["latency_ba"].max(),
        ),
    ]

    ax.plot(
        limits,
        limits,
        linestyle="--",
        linewidth=1.5,
        color="red",
    )

    ax.set_xlabel(
        "Latency A → B (ms)",
        fontsize=16,
        labelpad=10,
    )

    ax.set_ylabel(
        "Latency B → A (ms)",
        fontsize=16,
        labelpad=10,
    )

    ax.tick_params(
        axis="both",
        labelsize=13,
    )

    ax.set_title(
        "Directional latency asymmetry",
        fontsize=18,
        pad=15,
    )

    ax.set_aspect("equal", adjustable="box")

    plt.tight_layout()

    asymmetry_plot
    return (asymmetry_plot,)


@app.cell
def _(mo):
    mo.md(r"""
    ## HTTP Network Asymmetry
    """)
    return


@app.cell(hide_code=True)
def _(http_enriched, pl):
    http_directional_latency = (
        http_enriched
        .group_by(["src", "dst"])
        .agg(
            pl.col("rt").mean().alias("latency_ms")
        )
    )
    http_forward = http_directional_latency.rename(
        {
            "src": "node_a",
            "dst": "node_b",
            "latency_ms": "latency_ab",
        }
    )

    http_reverse = http_directional_latency.rename(
        {
            "src": "node_b",
            "dst": "node_a",
            "latency_ms": "latency_ba",
        }
    )

    http_asymmetry = (
        http_forward.join(
            http_reverse,
            on=["node_a", "node_b"],
            how="inner",
        )
        .filter(pl.col("node_a") < pl.col("node_b"))
        .with_columns(
            (pl.col("latency_ab") - pl.col("latency_ba"))
            .abs()
            .alias("asymmetry_ms")
        )
        .with_columns(
            (
                pl.col("asymmetry_ms")
                / ((pl.col("latency_ab") + pl.col("latency_ba")) / 2)
                * 100
            ).alias("asymmetry_percent")
        )
        .sort("asymmetry_ms", descending=True)
    )
    return (http_asymmetry,)


@app.cell(hide_code=True)
def _(http_asymmetry, plt, sns):
    http_asymmetry_plot, ax_http_asymmetry = plt.subplots(figsize=(8, 8))

    sns.scatterplot(
        data=http_asymmetry.to_pandas(),
        x="latency_ab",
        y="latency_ba",
        s=80,
        ax=ax_http_asymmetry,
    )

    http_limits = [
        min(
            http_asymmetry["latency_ab"].min(),
            http_asymmetry["latency_ba"].min(),
        ),
        max(
            http_asymmetry["latency_ab"].max(),
            http_asymmetry["latency_ba"].max(),
        ),
    ]

    ax_http_asymmetry.plot(
        http_limits,
        http_limits,
        linestyle="--",
        linewidth=1.5,
        color="red",
    )

    ax_http_asymmetry.set_xlabel(
        "Latency A → B (ms)",
        fontsize=16,
        labelpad=10,
    )

    ax_http_asymmetry.set_ylabel(
        "Latency B → A (ms)",
        fontsize=16,
        labelpad=10,
    )

    ax_http_asymmetry.tick_params(
        axis="both",
        labelsize=13,
    )

    ax_http_asymmetry.set_title(
        "HTTP directional latency asymmetry",
        fontsize=18,
        pad=15,
    )

    ax_http_asymmetry.set_aspect("equal", adjustable="box")

    plt.tight_layout()

    http_asymmetry_plot
    return (http_asymmetry_plot,)


@app.cell(hide_code=True)
def _(ping_enriched, pl, plt, sns):
    # 1. Per-run matched asymmetry (A->B paired with nearest-in-time B->A)
    ping_runs = (
        ping_enriched
        .select(["src", "dst", "datetime", "avg", "prb_id"])
        .drop_nulls(["avg", "datetime"])
    )
    ping_ab = ping_runs.rename({"src": "a", "dst": "b", "avg": "avg_ab"})
    ping_ba = ping_runs.rename({"src": "b", "dst": "a", "avg": "avg_ba"})

    per_run_asym_disagg = (
        ping_ab
        .sort("datetime")
        .join_asof(
            ping_ba.sort("datetime"),
            left_on="datetime",
            right_on="datetime",
            by=["a", "b"],
            strategy="nearest",
            tolerance="10m",
        )
        .filter(pl.col("avg_ba").is_not_null())
        .with_columns(
            (pl.col("avg_ab") - pl.col("avg_ba")).alias("delta_ms"),
            (
                (pl.col("avg_ab") - pl.col("avg_ba")).abs()
                / ((pl.col("avg_ab") + pl.col("avg_ba")) / 2)
                * 100
            ).alias("delta_pct"),
        )
    )

    # 2. Summarize per pair: is the asymmetry persistent or intermittent?
    asym_disagg_summary = (
        per_run_asym_disagg
        .group_by(["a", "b"])
        .agg(
            pl.len().alias("n_matched"),
            pl.col("delta_pct").mean().alias("mean_pct"),
            pl.col("delta_pct").median().alias("median_pct"),
            pl.col("delta_pct").quantile(0.9).alias("p90_pct"),
            (pl.col("delta_pct").abs() > 20).mean().alias("frac_gt_20pct"),
        )
        .with_columns(
            pl.when(pl.col("median_pct").abs() > 10)
            .then(pl.lit("Persistent"))
            .otherwise(pl.lit("Intermittent / noise"))
            .alias("regime")
        )
        .sort("p90_pct", descending=True)
    )

    # 3. Figure: per-pair per-run asymmetry distributions (top 20 pairs by p90)
    top_pairs = asym_disagg_summary.head(20).select(["a", "b"])
    asym_plot_data = (
        per_run_asym_disagg
        .join(top_pairs, on=["a", "b"], how="inner")
        .with_columns(pl.concat_str(["a", "b"], separator=" → ").alias("pair"))
        .to_pandas()
    )
    pair_order = (
        asym_disagg_summary
        .head(20)
        .with_columns(pl.concat_str(["a", "b"], separator=" → ").alias("pair"))
        ["pair"].to_list()
    )

    asym_disagg_fig, asym_disagg_ax = plt.subplots(figsize=(12, 10))

    sns.stripplot(
        data=asym_plot_data,
        y="pair",
        x="delta_pct",
        order=pair_order,
        size=4,
        alpha=0.5,
        color="#4C72B0",
        ax=asym_disagg_ax,
    )

    sns.boxplot(
        data=asym_plot_data,
        y="pair",
        x="delta_pct",
        order=pair_order,
        showfliers=False,
        boxprops=dict(facecolor="none", edgecolor="#C44E52"),
        whiskerprops=dict(color="#C44E52"),
        capprops=dict(color="#C44E52"),
        medianprops=dict(color="#C44E52", linewidth=2),
        ax=asym_disagg_ax,
    )

    asym_disagg_ax.axvline(0, color="black", linewidth=1)
    asym_disagg_ax.axvspan(-10, 10, color="gray", alpha=0.12,
                           label="±10% band")
    asym_disagg_ax.set_xlabel("Per-run directional asymmetry (%)", fontsize=15)
    asym_disagg_ax.set_ylabel("Node pair (A → B)", fontsize=15)
    asym_disagg_ax.set_title(
        "Per-run asymmetry: persistent vs intermittent routing",
        fontsize=17, pad=12,
    )
    asym_disagg_ax.tick_params(labelsize=11)
    asym_disagg_ax.grid(axis="x", linestyle=":", alpha=0.4)
    asym_disagg_ax.legend(fontsize=11, loc="lower right")
    plt.tight_layout()

    asym_disagg_fig
    return


@app.cell
def _(classify_http_error_robust, http_measurements, pl):
    unknown_errors = (
        http_measurements
        .filter(pl.col("err").is_not_null())
        .with_columns(
            pl.col("err")
            .map_elements(classify_http_error_robust, return_dtype=pl.String)
            .alias("cause")
        )
        .filter(pl.col("cause") == "Other")
        .group_by("err")
        .len()
        .sort("len", descending=True)
    )
    unknown_errors
    return


@app.cell(hide_code=True)
def _(http_enriched, pl, plt, sns):
    # 1. Per-run matched asymmetry (A->B paired with nearest-in-time B->A)
    http_runs = (
        http_enriched
        .select(["src", "dst", "datetime", "rt", "prb_id"])
        .drop_nulls(["rt", "datetime"])
    )
    http_ab = http_runs.rename({"src": "a", "dst": "b", "rt": "rt_ab"})
    http_ba = http_runs.rename({"src": "b", "dst": "a", "rt": "rt_ba"})

    per_run_asym_http = (
        http_ab
        .sort("datetime")
        .join_asof(
            http_ba.sort("datetime"),
            left_on="datetime",
            right_on="datetime",
            by=["a", "b"],
            strategy="nearest",
            tolerance="10m",
        )
        .filter(pl.col("rt_ba").is_not_null())
        .with_columns(
            (pl.col("rt_ab") - pl.col("rt_ba")).alias("delta_ms"),
            (
                (pl.col("rt_ab") - pl.col("rt_ba")).abs()
                / ((pl.col("rt_ab") + pl.col("rt_ba")) / 2)
                * 100
            ).alias("delta_pct"),
        )
    )

    # 2. Summarize per pair: is the asymmetry persistent or intermittent?
    asym_http_summary = (
        per_run_asym_http
        .group_by(["a", "b"])
        .agg(
            pl.len().alias("n_matched"),
            pl.col("delta_pct").mean().alias("mean_pct"),
            pl.col("delta_pct").median().alias("median_pct"),
            pl.col("delta_pct").quantile(0.9).alias("p90_pct"),
            (pl.col("delta_pct").abs() > 20).mean().alias("frac_gt_20pct"),
        )
        .with_columns(
            pl.when(pl.col("median_pct").abs() > 10)
            .then(pl.lit("Persistent"))
            .otherwise(pl.lit("Intermittent / noise"))
            .alias("regime")
        )
        .sort("p90_pct", descending=True)
    )

    # 3. Figure: per-pair per-run asymmetry distributions (top 20 pairs by p90)
    http_top_pairs = asym_http_summary.head(20).select(["a", "b"])
    http_asym_plot_data = (
        per_run_asym_http
        .join(http_top_pairs, on=["a", "b"], how="inner")
        .with_columns(pl.concat_str(["a", "b"], separator=" → ").alias("pair"))
        .to_pandas()
    )
    http_pair_order = (
        asym_http_summary
        .head(20)
        .with_columns(pl.concat_str(["a", "b"], separator=" → ").alias("pair"))
        ["pair"].to_list()
    )

    http_asym_disagg_fig, http_asym_disagg_ax = plt.subplots(figsize=(12, 10))

    sns.stripplot(
        data=http_asym_plot_data,
        y="pair",
        x="delta_pct",
        order=http_pair_order,
        size=4,
        alpha=0.5,
        color="#4C72B0",
        ax=http_asym_disagg_ax,
    )

    sns.boxplot(
        data=http_asym_plot_data,
        y="pair",
        x="delta_pct",
        order=http_pair_order,
        showfliers=False,
        boxprops=dict(facecolor="none", edgecolor="#C44E52"),
        whiskerprops=dict(color="#C44E52"),
        capprops=dict(color="#C44E52"),
        medianprops=dict(color="#C44E52", linewidth=2),
        ax=http_asym_disagg_ax,
    )

    http_asym_disagg_ax.axvline(0, color="black", linewidth=1)
    http_asym_disagg_ax.axvspan(-10, 10, color="gray", alpha=0.12,
                                label="±10% band")
    http_asym_disagg_ax.set_xlabel("Per-run directional asymmetry (%)", fontsize=15)
    http_asym_disagg_ax.set_ylabel("Node pair (A → B)", fontsize=15)
    http_asym_disagg_ax.set_title(
        "HTTP per-run asymmetry: persistent vs intermittent routing",
        fontsize=17, pad=12,
    )
    http_asym_disagg_ax.tick_params(labelsize=11)
    http_asym_disagg_ax.grid(axis="x", linestyle=":", alpha=0.4)
    http_asym_disagg_ax.legend(fontsize=11, loc="lower right")
    plt.tight_layout()

    http_asym_disagg_fig
    return


@app.cell
def _(mo):
    mo.md(r"""
    # Network Map
    """)
    return


@app.cell(hide_code=True)
def _(metadata, ping_enriched, pl):
    import geopandas as gpd
    from matplotlib.colors import Normalize
    from matplotlib.cm import ScalarMappable
    from shapely.geometry import LineString

    # Node coordinates based on the project's metadata table
    map_nodes = (
        metadata
        .select(
            pl.col("location").alias("node"),
            pl.col("lon").cast(pl.Float64).alias("longitude"),
            pl.col("lat").cast(pl.Float64).alias("latitude"),
        )
        .unique(subset=["node"])
    )

    # Average RTT for each directed source-destination pair
    map_pairs = (
        ping_enriched
        .group_by(["src", "dst"])
        .agg(pl.col("avg").mean().alias("latency_ms"))
        .to_pandas()
    )

    map_pairs = map_pairs.loc[map_pairs["src"] != map_pairs["dst"]].copy()

    # Build a lookup table for node coordinates
    node_coords = map_nodes.to_pandas().set_index("node")[
        ["longitude", "latitude"]
    ]

    # Attach source and destination coordinates
    map_pairs = map_pairs.join(
        node_coords.add_prefix("src_"),
        on="src",
    ).join(
        node_coords.add_prefix("dst_"),
        on="dst",
    )

    # Drop pairs without coordinates
    map_pairs = map_pairs.dropna(
        subset=[
            "src_longitude",
            "src_latitude",
            "dst_longitude",
            "dst_latitude",
            "latency_ms",
        ]
    )

    # Create geographic lines
    map_pairs["geometry"] = map_pairs.apply(
        lambda row: LineString([
            (row["src_longitude"], row["src_latitude"]),
            (row["dst_longitude"], row["dst_latitude"]),
        ]),
        axis=1,
    )

    connection_lines = gpd.GeoDataFrame(
        map_pairs,
        geometry="geometry",
        crs="EPSG:4326",
    )

    # Keep only nodes inside the displayed extent
    nodes_europe = map_nodes.to_pandas().loc[
        lambda df: df["longitude"].between(-12, 35)
        & df["latitude"].between(34, 72)
    ]
    return Normalize, ScalarMappable, connection_lines, gpd, nodes_europe


@app.cell(hide_code=True)
def _(Normalize, connection_lines, gpd, plt):
    # Country boundaries from Natural Earth
    world_map = gpd.read_file(
        "https://naturalearth.s3.amazonaws.com/"
        "50m_cultural/ne_50m_admin_0_countries.zip"
    )

    # Europe-wide extent, including Great Britain and Ireland
    euro_map = world_map.cx[-12:35, 34:72]

    # Set a shared color scale for RTT
    vmin = connection_lines["latency_ms"].min()
    vmax = connection_lines["latency_ms"].max()

    if vmin == vmax:
        vmax = vmin + 1

    norm = Normalize(vmin=vmin, vmax=vmax)
    cmap = plt.get_cmap("viridis")
    return cmap, euro_map, norm


@app.cell(hide_code=True)
def _(
    ScalarMappable,
    cmap,
    connection_lines,
    euro_map,
    nodes_europe,
    norm,
    plt,
):
    # Create figure
    euro_network_plot, ax_euro_network = plt.subplots(
        figsize=(14, 11)
    )

    # Country basemap
    euro_map.plot(
        ax=ax_euro_network,
        color="#F1F1F1",
        edgecolor="#A0A0A0",
        linewidth=0.6,
    )

    # Connections, colored by average RTT
    for _, row in connection_lines.iterrows():
        ax_euro_network.plot(
            [row["src_longitude"], row["dst_longitude"]],
            [row["src_latitude"], row["dst_latitude"]],
            color=cmap(norm(row["latency_ms"])),
            linewidth=1.2,
            alpha=0.55,
            zorder=2,
        )

    # Measurement nodes
    ax_euro_network.scatter(
        nodes_europe["longitude"],
        nodes_europe["latitude"],
        s=55,
        color="#D62728",
        edgecolor="white",
        linewidth=0.8,
        zorder=3,
    )

    # Optional: label nodes if there are not too many
    if len(nodes_europe) <= 25:
        for _, node in nodes_europe.iterrows():
            ax_euro_network.annotate(
                str(node["node"]),
                (node["longitude"], node["latitude"]),
                xytext=(4, 4),
                textcoords="offset points",
                fontsize=8,
                zorder=4,
            )

    # Colorbar
    scalar_map = ScalarMappable(norm=norm, cmap=cmap)
    scalar_map.set_array([])

    colorbar = euro_network_plot.colorbar(
        scalar_map,
        ax=ax_euro_network,
        shrink=0.72,
        pad=0.02,
    )
    colorbar.set_label("Average RTT (ms)", fontsize=12)
    colorbar.ax.tick_params(labelsize=10)

    ax_euro_network.set_xlim(-12, 35)
    ax_euro_network.set_ylim(34, 72)
    ax_euro_network.set_xlabel("Longitude (°)", fontsize=12)
    ax_euro_network.set_ylabel("Latitude (°)", fontsize=12)
    ax_euro_network.set_title(
        "European Network: Geographic Distribution and RTT",
        fontsize=16,
        pad=14,
    )
    ax_euro_network.grid(
        True,
        linestyle=":",
        alpha=0.3,
    )
    ax_euro_network.set_aspect("auto")

    plt.tight_layout()
    euro_network_plot
    return (euro_network_plot,)


@app.cell
def _(
    Path,
    asymmetry_plot,
    distance_ping_plot,
    euro_network_plot,
    failure_plot,
    http_asymmetry_plot,
    http_distance_ping_plot,
    http_heatmap_raw,
    mo,
    ping_heatmap_raw,
):
    # Directory where the notebook lives
    figure_dir = Path("notebooks/figures/")

    # Map of descriptive names to the figures created in this notebook
    figures_to_export = {
        "distance_ping": distance_ping_plot,
        "http_distance_ping": http_distance_ping_plot,
        "ping_heatmap_raw": ping_heatmap_raw,
        "http_heatmap_raw": http_heatmap_raw,
        "asymmetry": asymmetry_plot,
        "http_asymmetry": http_asymmetry_plot,
        "euro_network": euro_network_plot,
        "failures": failure_plot,
    }

    for name, fig in figures_to_export.items():
        fig.savefig(
            figure_dir / f"{name}.png",
            dpi=300,
            bbox_inches="tight",
        )
        fig.savefig(
            figure_dir / f"{name}.svg",
            bbox_inches="tight",
        )

    exported_figure_names = list(figures_to_export)
    mo.md(
        "**Exported figures to "
        f"`{figure_dir}`:** "
        + ", ".join(f"`{n}`" for n in exported_figure_names)
    )
    return


if __name__ == "__main__":
    app.run()
