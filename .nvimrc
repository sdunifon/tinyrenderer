
autocmd VimEnter * echo 'local .nvim file loaded'
" autocmd BufEnter * echo  @%


command Compile :!cargo build
command Test :!cargo test
command Play :!cargo play %
noremap <F5> :Compile<CR> 
noremap <F6> :Test<CR>,

unmap <leader>;

noremap <leader>; mnA;<esc>`n
noremap <leader>, mnA,<esc>`n

noremap <leader>d; mnf;x`n
noremap <leader>d, mnf,x`n

noremap <leader>hh :echo hello<CR>
noremap <leader>de :e .nvimrc<CR> 
