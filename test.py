import fastcp as fcp

result, val_or_err = fcp.read_number()

if result:
    print("You entered: ",val_or_err)
else:
    print("Error raised: ",val_or_err)

