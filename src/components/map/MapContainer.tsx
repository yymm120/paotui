import {
    Map,
    APILoader,
    ScaleControl,
    ToolBarControl,
    ControlBarControl,
    Geolocation,
    Marker
} from "@uiw/react-amap";
import "./styles.css";

function Demo() {
    return (
        <div style={{ width: "100%", height: "300px" }}>
            <Map zoom={4}>
                <ScaleControl offset={[16, 30]} position="LB" />
                <ToolBarControl offset={[16, 10]} position="RB" />
                <ControlBarControl offset={[16, 180]} position="RB" />
                <Geolocation
                    maximumAge={100000}
                    borderRadius="5px"
                    position="RB"
                    offset={[16, 80]}
                    zoomToAccuracy={true}
                    showCircle={true}
                />
                <Marker
                    title="北京市"
                    // offset={new AMap.Pixel(-13, -30)}
                    position={[117.283042, 31.86119]}
                >
                    <div style={{ backgroundColor: "#333", width: 200, color: "white" }}>
                        我是 marker 的 label 标签
                    </div>
                </Marker>
            </Map>
            <Map>
                {({ AMap, map, container }) => {
                    return;
                }}
            </Map>
        </div>
    );
}

export default function App() {
    return (
        <APILoader akey={"0d0f0a9438774885a0e28160a9d76b7d"} >
            <Demo />
        </APILoader>
    );
}
