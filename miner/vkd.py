import json
import os
import pickle

import itertools
import requests
import time

import sys

MAX_FAILURE_COUNT = 3
WORK_PATH = ""
INIT_PATH = ""
MAX_REQUEST_SIZE = 250

UNSTARTED_STAGE = -1
FRIENDS_STAGE = 0
META_STAGE = 1

ALL_IDS = {}
IDS = {}


def save_state(dir, stage):
    with open(os.path.join(dir, "state.pickle"), "wb") as state_file:
        pickle.dump(ALL_IDS, state_file)
        pickle.dump(IDS, state_file)
        pickle.dump(stage, state_file)


def load_state(dir):
    global ALL_IDS
    global IDS
    try:
        with open(os.path.join(dir, "state.pickle"), 'rb') as state_file:
            ALL_IDS = pickle.load(state_file)
            IDS = pickle.load(state_file)
            return pickle.load(state_file)
    except:
        return UNSTARTED_STAGE


if __name__ == "__main__":
    print(time.strftime("%H:%M:%S"))
    stage = load_state(WORK_PATH)
    if stage == UNSTARTED_STAGE:
        with open(INIT_PATH) as ids_file:
            for sid in ids_file:
                id = int(sid)
                IDS[id] = True
                ALL_IDS[id] = {}
        stage = FRIENDS_STAGE
    if stage == FRIENDS_STAGE:
        print("Getting friends")
        failure = 0
        count = len(IDS)
        while IDS:
            id, follow_friends = IDS.popitem()
            if follow_friends:
                count -= 1
                print("%d (%d left)" % (id, count))
                r = requests.get("https://api.vk.com/method/friends.get?user_id=%d" % id)
                if r.status_code == 200:
                    js = r.json()
                    if "error" in js:
                        del ALL_IDS[id]
                        continue
                    fs = [int(f) for f in r.json()["response"]]
                    ALL_IDS[id]["friends"] = fs
                    for friend in fs:
                        if friend not in ALL_IDS:
                            IDS[friend] = False
                            ALL_IDS[friend] = {}
                    save_state(WORK_PATH, FRIENDS_STAGE)
                else:
                    print("API request error: " + str(r.status_code))
                    failure += 1
                    if failure == MAX_FAILURE_COUNT:
                        print("Max request error reached")
                        sys.exit(1)
                    else:
                        time.sleep(10)
                time.sleep(5)
        IDS = {id: None for id in ALL_IDS}
        save_state(WORK_PATH, META_STAGE)
        stage = META_STAGE
    if stage == META_STAGE:
        print("Getting meta")
        failure = 0
        count = len(IDS) / MAX_REQUEST_SIZE + 1
        while IDS:
            ids = [str(id) for id in itertools.islice(IDS, min(MAX_REQUEST_SIZE, len(IDS)))]
            params = {
                "user_ids": ",".join(ids),
                "fields": "photo_id,verified,sex,bdate,city,country,home_town,has_photo,photo_50,photo_100,"
                          "photo_200_orig,photo_200,photo_400_orig,photo_max,photo_max_orig,online,lists,domain,"
                          "has_mobile,contacts,site,education,universities,schools,status,last_seen,followers_count,"
                          "occupation,nickname,relatives,relation,personal,connections,exports,wall_comments,"
                          "activities,interests,music,movies,tv,books,games,about,quotes,can_post,can_see_all_posts,"
                          "can_see_audio,can_write_private_message,can_send_friend_request,is_favorite,"
                          "is_hidden_from_feed,timezone,screen_name,maiden_name,crop_photo,is_friend,friend_status,"
                          "career,military,blacklisted,blacklisted_by_me"
            }
            count -= 1
            print("%d" % count)
            r = requests.post("https://api.vk.com/method/users.get", params=params)
            if r.status_code == 200:
                js = r.json()
                if "error" in js:
                    print(js)
                    sys.exit(2)
                for meta in js["response"]:
                    id = int(meta["uid"])
                    if "deactivated" not in meta:
                        ALL_IDS[id].update(meta)
                    else:
                        del ALL_IDS[id]
                    del IDS[id]
                save_state(WORK_PATH, META_STAGE)
                time.sleep(5)
            else:
                print("API request error: " + str(r.status_code))
                print(params)
                failure += 1
                if failure == MAX_FAILURE_COUNT:
                    print("Max request error reached")
                    sys.exit(1)
                else:
                    time.sleep(10)
    with open(os.path.join(WORK_PATH, "result.data"), 'w') as result_file:
        for user in ALL_IDS.values():
            json.dump(user, result_file, ensure_ascii=False)
            result_file.write("\n")
    os.remove(os.path.join(WORK_PATH, "state.pickle"))
    print(time.strftime("%H:%M:%S"))
