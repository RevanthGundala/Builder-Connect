export async function check_session(): Promise<string | undefined> {
  try {
    const url = process.env.NEXT_PUBLIC_BASE_URL + `/get_session`;
    const res = await fetch(url, { credentials: "include" });
    const data = await res.json();
    return data === "Not set." ? "" : data;
  } catch (err) {
    console.log(err);
  }
}

export async function view_profile(
  sub_id: string,
  profile: any
): Promise<any | undefined> {
  try {
    if (!sub_id) return profile;
    const url = process.env.NEXT_PUBLIC_BASE_URL + `/view/${sub_id}`;
    const res = await fetch(url, { credentials: "include" });
    const data = await res.json();
    return JSON.stringify(profile) !== JSON.stringify(data) ? data : profile;
  } catch (err) {
    console.log(err);
  }
}
