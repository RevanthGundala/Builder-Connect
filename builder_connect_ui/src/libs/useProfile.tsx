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

// This will read the profile if there is an ID passed in, or it will default to the current user's profile.
export default function useReadProfile(id: string | undefined = undefined) {
  const [profile, set_profile] = useState<any | undefined>(undefined);
  let { sub_id } = useReadSession();
  if (id) sub_id = id;
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
  return [profile, set_profile];
}
