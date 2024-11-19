export async function apiRequest(
  url: string,
  method: 'GET' | 'POST' | 'PUT' | 'DELETE',
  body?: {},
  headers?: HeadersInit
) {
  const response = await fetch(`http://localhost:8080${url}`, {
    method,
    headers: {
      'Content-Type': 'application/json',
      ...headers
    },
    body: body ? JSON.stringify(body) : undefined
  })

  if (!response.ok) {
    const errorMessage: { error_code: number; message: string } = await response.json()
    throw new Error(
      errorMessage.message
        ? `Error ${response.status}: ${errorMessage.message}`
        : `Ha ocurrido un error inesperado`
    )
  }
  if (response.statusText === 'No Content') return null
  return response.json()
}
