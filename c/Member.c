#ifndef ___Member
#define ___Member

typedef struct {
    int  no;
    char name[20];
} Member;

#define MEMBER_NO
#define MEMBER_NAME

int MemberCmp(const Member *x, const Member *y);

int MemberNameCmp(const Member *x, const Member *y);

void PrintMember(const Member *x);

void PrintLnMember(const Member *x);

Member ScanMember(const char *message, int sw);

#endif



