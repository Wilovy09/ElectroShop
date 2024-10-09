export async function apiRequest(
  url: string,
  method: "GET" | "POST" | "PUT" | "DELETE",
  body?: {},
  headers?: HeadersInit
) {
  const puerto = 8150;
  const response = await fetch(`http://localhost:${puerto}${url}`, {
    method,
    headers: {
      "Content-Type": "application/json",
      ...headers,
    },
    body: body ? JSON.stringify(body) : undefined,
  });

  if (!response.ok) {
    const errorMessage = await response.text();
    throw new Error(`Error ${response.status}: ${errorMessage}`);
  }

  return response.json();
}
