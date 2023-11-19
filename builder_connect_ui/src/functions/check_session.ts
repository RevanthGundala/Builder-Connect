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
