import { useEffect, useState } from "react";

async function read_session(): Promise<string | undefined> {
  try {
    const url = process.env.NEXT_PUBLIC_BASE_URL + `/get_session`;
    const res = await fetch(url, { credentials: "include" });
    const data = await res.json();
    return data === "Not set." ? "" : data;
  } catch (err) {
    console.log(err);
  }
}

export default function useReadSession() {
  const [session_data, set_session_data] = useState<string | undefined>(
    undefined
  );
  useEffect(() => {
    const controller = new AbortController();
    read_session()
      .then((data) => {
        console.log("Session data: ", data);
        if (data !== session_data) set_session_data(data);
      })
      .catch((e) => console.log(e));
    return () => controller.abort();
  }, [session_data]);
  return session_data;
}
