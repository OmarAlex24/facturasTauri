import { useState, useEffect } from "react";
import { getVersion } from "@tauri-apps/api/app";

const VersionDisplay = () => {
  const [version, setVersion] = useState("");

  useEffect(() => {
    getVersion().then(setVersion);
  }, []);
  return <div>Version: {version}</div>;
};

export default VersionDisplay;
