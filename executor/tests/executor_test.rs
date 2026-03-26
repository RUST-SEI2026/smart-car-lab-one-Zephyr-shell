use executor::{Executor,Pose};
mod move_tests{
    use super::*;

    #[test]
    #[ignore]
    fn should_return_x_plus_1_given_command_is_m_and_facing_is_e(){

        let original_pose = Pose::new(0,0,'E');

        let mut executor = Executor::with_pose(original_pose);

        executor.execute("M");

        let expected_pose = Pose::new(1,0,'E');
        assert_eq!(expected_pose,executor.query());
    }

    #[test]
    fn should_return_y_plus_1_given_command_is_m_and_facing_is_n(){

        let original_pose = Pose::new(0,0,'N');

        let mut executor = Executor::with_pose(original_pose);

        executor.execute("M");

        let expected_pose = Pose::new(0,1,'N');
        assert_eq!(expected_pose,executor.query());
    }


}
