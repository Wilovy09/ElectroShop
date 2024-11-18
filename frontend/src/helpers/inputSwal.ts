import Swal from "sweetalert2";

export async function showInputSwal(title: string) {
  const inputValue = await Swal.fire({
    title: title,
    input: "text",
    showCancelButton: true,
    confirmButtonColor: "#059669",
    confirmButtonText: "Aceptar",
    cancelButtonText: "Cancelar",
    inputValidator: (value) => {
      if (!value) {
        return "Escriba un nombre por favor";
      }
    },
    customClass: {
      popup: "colored-toast",
      title: "colored-modal-title",
      input: "colored-modal-input",
      validationMessage: "colored-modal-validation",
    },
  });
  return inputValue.value;
}

const style = document.createElement("style");
style.textContent = `
  .colored-toast {
    background-color: #1e293b !important;
    color: white !important;
  }
  
  .colored-modal-title {
    color: white !important;
  }
  
  .colored-modal-input {
    background-color: transparent !important;
    border: 1px solid #e2e8f0 !important;
    color: white !important;
    border-radius: 0.375rem !important;
  }
  
  .colored-modal-input:focus {
    outline: none !important;
    border-color: #e2e8f0 !important;
  }
  
  .colored-modal-validation {
    color: #ef4444 !important;
  }
`;
document.head.appendChild(style);

export default showInputSwal;
