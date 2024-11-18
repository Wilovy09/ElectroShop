import Swal from "sweetalert2";

const showSuccedSwal = (message: string): void => {
  Swal.mixin({
    toast: true,
    position: "top-end",
    showConfirmButton: false,
    timer: 3000,
    timerProgressBar: true,
    didOpen: (toast) => {
      toast.addEventListener("mouseenter", Swal.stopTimer);
      toast.addEventListener("mouseleave", Swal.resumeTimer);
    },
    customClass: {
      popup: "colored-toast",
    },
    iconColor: "",
  }).fire({
    icon: "success",
    text: message,
  });
};

const style = document.createElement("style");
style.textContent = `
  .colored-toast.swal2-icon-success {
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
`;
document.head.appendChild(style);

export default showSuccedSwal;
