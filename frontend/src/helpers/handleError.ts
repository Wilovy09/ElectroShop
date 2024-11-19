import Swal from 'sweetalert2'

interface ErrorResponse {
  message: string
  code?: string | number
}

const handleError = (error: unknown): void => {
  let errorMessage: string
  if (typeof error === 'string') {
    errorMessage = error
  } else if (error instanceof Error) {
    errorMessage = error.message
  } else if (typeof error === 'object' && error !== null) {
    const errorObj = error as ErrorResponse
    errorMessage = errorObj.message || 'An unknown error occurred'
  } else {
    errorMessage = 'An unknown error occurred'
  }

  Swal.mixin({
    toast: true,
    position: 'top-end',
    showConfirmButton: false,
    timer: 3000,
    timerProgressBar: true,
    didOpen: (toast) => {
      toast.addEventListener('mouseenter', Swal.stopTimer)
      toast.addEventListener('mouseleave', Swal.resumeTimer)
    },
    customClass: {
      popup: 'colored-toast'
    },
    iconColor: ''
  }).fire({
    icon: 'error',
    text: errorMessage
  })
}

const style = document.createElement('style')
style.textContent = `
  .colored-toast.swal2-icon-error {
    background-color: #1e293b !important;
    color: white !important;
  }
  .colored-toast .swal2-title {
    color: white !important;
  }
  .colored-toast .swal2-close {
    color: white !important;
  }
  .colored-toast .swal2-html-container {
    color: #e2e8f0 !important;
  }
`
document.head.appendChild(style)

export default handleError
