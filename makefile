test:
	bash scripts/test.sh

coveralls:
	echo "Running coveralls for Job ID: ${JOB_ID}"
	bash scripts/coveralls.sh ${JOB_ID}
