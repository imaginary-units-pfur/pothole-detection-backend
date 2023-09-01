from flask import Flask, render_template_string
import folium
import pandas as pd
from database import Database

app = Flask(__name__)


def add_marker(row, map):
    folium.Marker(
        location=[row["latitude"], row["longitude"]],
        tooltip="Potholes",
        icon=folium.Icon(icon="cone", color="orange"),
    ).add_to(map)


m = folium.Map(location=[43.0403, -76.1378])  # the City of Syracuse
potholes_data = pd.read_csv(
    "./data/cityofsyracuse-potholes-filled/potholes_with_tnt.csv"
)
potholes_data.apply(add_marker, axis=1, args=(m,))


@app.route("/")
def iframe():
    """Embed a map as an iframe on a page."""

    # set the iframe width and height
    m.get_root().width = "1600px"
    m.get_root().height = "600px"
    iframe = m.get_root()._repr_html_()

    return render_template_string(
        """
            <!DOCTYPE html>
            <html>
                <head></head>
                <body>
                    <h1>Pothole data</h1>
                    {{ iframe|safe }}
                </body>
            </html>
        """,
        iframe=iframe,
    )


if __name__ == "__main__":
    app.run(debug=True)
