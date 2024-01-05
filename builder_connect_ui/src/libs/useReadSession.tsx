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
  const [sub_id, set_sub_id] = useState<string | undefined>(undefined);
  const [is_loading, set_is_loading] = useState(false);
  useEffect(() => {
    const controller = new AbortController();
    set_is_loading(true);
    read_session()
      .then((data) => {
        console.log("Session data: ", data);
        if (data !== sub_id) set_sub_id(data);
      })
      .catch((e) => console.log(e));
    return () => {
      controller.abort();
      set_is_loading(false);
    };
  }, [sub_id]);
  return { is_loading, sub_id };
}
