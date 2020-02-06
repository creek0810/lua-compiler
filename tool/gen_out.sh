all_file=`ls ../tests/*.lua`
for entry in $all_file
do
  out_name=${entry/lua/"out"}
  luac -o $out_name $entry
done