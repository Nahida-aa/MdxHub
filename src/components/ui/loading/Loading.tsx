export const LoadingS = () => {
  return <div>
    <svg viewBox='0 0 200 200'>
      <circle cx="100" cy="100" r="50" fill='none' strokeWidth="10px" strokeLinecap='round' className='animate-[neon-loading_2s_linear_infinite] blur-[10px] stroke-[#43acff]' />
      <circle cx="100" cy="100" r="50" fill='none'  strokeWidth="10px" strokeLinecap='round' className='animate-[neon-loading_2s_linear_infinite] blur-[20px] stroke-[#43acff]' />
      <circle cx="100" cy="100" r="50" fill='none' strokeWidth="10px" strokeLinecap='round' className='animate-[neon-loading_2s_linear_infinite] stroke-[#fff]' />
    </svg>
  </div>
}