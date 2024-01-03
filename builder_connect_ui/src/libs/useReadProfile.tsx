import { useEffect, useState } from "react";
import useReadSession from "./useReadSession";

async function view_profile(sub_id: string): Promise<any | undefined> {
  try {
    const url = process.env.NEXT_PUBLIC_BASE_URL + `/view/${sub_id}`;
    const res = await fetch(url, { credentials: "include" });
    const data = await res.json();
    return data;
  } catch (err) {
    console.log(err);
  }
}

export default function useReadProfile() {
  const [profile, set_profile] = useState<any | undefined>(undefined);
  const { is_loading, sub_id } = useReadSession();
  useEffect(() => {
    if (!sub_id) return profile;
    const controller = new AbortController();
    view_profile(sub_id)
      .then((data) => {
        console.log("Profile data: ", data);
        if (JSON.stringify(data) !== JSON.stringify(profile)) set_profile(data);
      })
      .catch((e) => console.log(e));
    return () => controller.abort();
  }, [profile, sub_id]);
  return profile;
}
